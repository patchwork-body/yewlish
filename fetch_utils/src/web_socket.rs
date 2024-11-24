use std::{cell::RefCell, rc::Rc};

use serde::Serialize;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, Event, MessageEvent, WebSocket};
use yew::Callback;

use crate::{deserialize_response, FetchError};

#[derive(Debug, PartialEq, Clone)]
pub struct WebSocketSubscriber<T>
where
    T: for<'de> serde::Deserialize<'de> + Clone + PartialEq + 'static,
{
    pub onopen: Callback<Event>,
    pub onmessage: Callback<(MessageEvent, T)>,
    pub onerror: Callback<FetchError>,
    pub onclose: Callback<CloseEvent>,
}

#[derive(Debug, PartialEq)]
pub struct WebSocketWatcher<T>
where
    T: for<'de> serde::Deserialize<'de> + Clone + PartialEq + 'static,
{
    url: String,
    ws: Option<WebSocket>,
    subscribers: Rc<RefCell<Vec<WebSocketSubscriber<T>>>>,
}

impl<T> WebSocketWatcher<T>
where
    T: for<'de> serde::Deserialize<'de> + Clone + PartialEq + 'static,
{
    #[must_use]
    pub fn new(url: String) -> Self {
        Self {
            url,
            ws: None,
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn subscribe(&mut self, subscriber: WebSocketSubscriber<T>) -> Result<(), FetchError> {
        (*self.subscribers).borrow_mut().push(subscriber);

        if self.ws.is_none() {
            self.ws = WebSocket::new(&self.url)
                .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?
                .into();
        }

        let Some(ws) = self.ws.as_ref() else {
            return Err(FetchError::NetworkError(
                "WebSocket connection is not established".to_string(),
            ));
        };

        let subscribers = self.subscribers.clone();

        let onopen_callback = Closure::wrap(Box::new(move |event: Event| {
            for subscriber in subscribers.borrow().iter() {
                subscriber.onopen.emit(event.clone());
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        let subscribers = self.subscribers.clone();

        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            match event.data().dyn_into::<web_sys::js_sys::JsString>() {
                Ok(message) => match deserialize_response::<T>(String::from(message).as_str()) {
                    Ok(res) => {
                        for subscriber in subscribers.borrow().iter() {
                            subscriber.onmessage.emit((event.clone(), res.clone()));
                        }
                    }
                    Err(err) => {
                        for subscriber in subscribers.borrow().iter() {
                            subscriber.onerror.emit(err.clone());
                        }
                    }
                },
                Err(err) => {
                    for subscriber in subscribers.borrow().iter() {
                        subscriber
                            .onerror
                            .emit(FetchError::ResponseDeserializationError(format!("{err:?}")));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        let subscribers = self.subscribers.clone();

        let onerror_callback = Closure::wrap(Box::new(move |event: ErrorEvent| {
            let error = FetchError::NetworkError(event.message().to_string());

            for subscriber in subscribers.borrow().iter() {
                subscriber.onerror.emit(error.clone());
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        let subscribers = self.subscribers.clone();

        let onclose_callback = Closure::wrap(Box::new(move |event: CloseEvent| {
            for subscriber in subscribers.borrow().iter() {
                subscriber.onclose.emit(event.clone());
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        Ok(())
    }

    pub fn unsubscribe(&mut self, subscriber: &WebSocketSubscriber<T>) -> Result<(), FetchError> {
        (*self.subscribers).borrow_mut().retain(|s| s != subscriber);

        if self.subscribers.borrow().is_empty() {
            let Some(ws) = self.ws.as_ref() else {
                return Ok(());
            };

            if ws.ready_state() == WebSocket::OPEN {
                ws.close()
                    .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;
            }
        }

        Ok(())
    }

    pub fn send<B>(&self, data: &B) -> Result<(), FetchError>
    where
        B: Serialize + Default + PartialEq,
    {
        let body_str = serde_json::to_string(&data)
            .map_err(|error| FetchError::BodySerializationError(error.to_string()))?;

        let Some(ws) = self.ws.as_ref() else {
            return Err(FetchError::NetworkError(
                "WebSocket connection is not established".to_string(),
            ));
        };

        if ws.ready_state() != WebSocket::OPEN {
            return Err(FetchError::NetworkError(
                "WebSocket connection is not open".to_string(),
            ));
        }

        ws.send_with_str(&body_str)
            .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

        Ok(())
    }

    #[must_use]
    pub fn get_ready_state(&self) -> Option<u16> {
        let ws = self.ws.as_ref()?;
        ws.ready_state().into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum WsStatus {
    Open,
    #[default]
    Closed,
}
