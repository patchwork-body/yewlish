use std::{cell::RefCell, rc::Rc};

use serde::Serialize;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, Event, MessageEvent, WebSocket};
use yew::Callback;

use crate::{build_url, FetchError};

pub struct WebSocketOptions<S, Q> {
    pub slugs: S,
    pub query: Q,
    pub onopen: Option<Callback<Event>>,
    pub onmessage: Option<Callback<MessageEvent>>,
    pub onerror: Option<Callback<ErrorEvent>>,
    pub onclose: Option<Callback<CloseEvent>>,
}

pub type SendMessage<B> = Callback<B, Result<(), FetchError>>;
pub type CloseConnection = Callback<(), Result<(), FetchError>>;

pub fn web_socket<S, Q, B>(
    url: &str,
    options: WebSocketOptions<S, Q>,
) -> Result<(SendMessage<B>, CloseConnection), FetchError>
where
    S: Serialize + Default + PartialEq,
    Q: Serialize + Default + PartialEq,
    B: Serialize + Default + PartialEq,
{
    let url = build_url(url, &options.slugs, &options.query)?;

    let ws = WebSocket::new(String::from(url.to_string()).as_str())
        .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

    let onopen_callback = Closure::wrap(Box::new(move |event: Event| {
        if let Some(onopen) = options.onopen.as_ref() {
            onopen.emit(event);
        }
    }) as Box<dyn FnMut(_)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(onmessage) = options.onmessage.as_ref() {
            onmessage.emit(event);
        }
    }) as Box<dyn FnMut(_)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |event: ErrorEvent| {
        if let Some(onerror) = options.onerror.as_ref() {
            onerror.emit(event);
        }
    }) as Box<dyn FnMut(_)>);

    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onclose_callback = Closure::wrap(Box::new(move |event: CloseEvent| {
        if let Some(onclose) = options.onclose.as_ref() {
            onclose.emit(event);
        }
    }) as Box<dyn FnMut(_)>);

    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();

    let send: SendMessage<B> = Callback::from({
        let ws = ws.clone();

        move |data: B| {
            let body_str = serde_json::to_string(&data)
                .map_err(|error| FetchError::BodySerializationError(error.to_string()))?;

            ws.send_with_str(&body_str)
                .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

            Ok(())
        }
    });

    let close: CloseConnection = Callback::from(move |()| {
        ws.close()
            .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

        Ok(())
    });

    Ok((send, close))
}

#[derive(Debug, PartialEq, Clone)]
pub struct WebSocketSubscriber {
    pub onopen: Callback<Event>,
    pub onmessage: Callback<MessageEvent>,
    pub onerror: Callback<ErrorEvent>,
    pub onclose: Callback<CloseEvent>,
}

#[derive(Debug, PartialEq)]
pub struct WebSocketWatcher {
    url: String,
    ws: Option<WebSocket>,
    subscribers: Rc<RefCell<Vec<WebSocketSubscriber>>>,
}

impl WebSocketWatcher {
    #[must_use]
    pub fn new(url: String) -> Self {
        Self {
            url,
            ws: None,
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn subscribe(&mut self, subscriber: WebSocketSubscriber) -> Result<(), FetchError> {
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
            for subscriber in subscribers.borrow().iter() {
                subscriber.onmessage.emit(event.clone());
            }
        }) as Box<dyn FnMut(_)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        let subscribers = self.subscribers.clone();

        let onerror_callback = Closure::wrap(Box::new(move |event: ErrorEvent| {
            for subscriber in subscribers.borrow().iter() {
                subscriber.onerror.emit(event.clone());
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

    pub fn unsubscribe(&mut self, subscriber: &WebSocketSubscriber) -> Result<(), FetchError> {
        (*self.subscribers).borrow_mut().retain(|s| s != subscriber);

        if self.subscribers.borrow().is_empty() {
            let Some(ws) = self.ws.as_ref() else {
                return Ok(());
            };

            ws.close()
                .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

            self.ws = None;
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

        ws.send_with_str(&body_str)
            .map_err(|error| FetchError::NetworkError(format!("{error:?}")))?;

        Ok(())
    }
}
