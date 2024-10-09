use std::{any::TypeId, cell::RefCell, rc::Rc, sync::Arc};

use serde::de::Deserialize;
use serde::Serialize;
use thiserror::Error;
use web_sys::{Headers, RequestInit};

use crate::{
    utils::{build_request, build_url, generate_cache_key, send_request},
    CachePolicy,
};

/// Enum representing supported HTTP methods.
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

impl From<String> for HttpMethod {
    fn from(method: String) -> Self {
        match method.to_uppercase().as_str() {
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            _ => HttpMethod::GET,
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        HttpMethod::from(method.to_string())
    }
}

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Url parsing error: {0}")]
    UrlParsingError(String),
    #[error("Slugs serialization error: {0}")]
    SlugsSerializationError(String),
    #[error("Query deserialization error: {0}")]
    QuerySerializationError(String),
    #[error("Header initialization error: {0}")]
    HeaderInitializationError(String),
    #[error("Header mutation error: {0}")]
    HeaderMutationError(String),
    #[error("Body serialization error: {0}")]
    BodySerializationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Window not found")]
    WindowNotFound(),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Http error: {0}")]
    HttpError(String),
    #[error("Json error: {0}")]
    JsonError(String),
    #[error("Response deserialization error: {0}")]
    ResponseDeserializationError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

pub type Middleware = Arc<Box<dyn Fn(&mut RequestInit, &mut Headers)>>;

/// Asynchronous function to perform an HTTP request using `web_sys` fetch API.
///
/// # Parameters
/// - `method`: The HTTP method to use for the request.
/// - `url`: The endpoint URL.
/// - `slugs`: Optional path parameters that implement `Serialize`.
/// - `query`: Optional query parameters that implement `Serialize`.
/// - `body`: Optional request body that implements `Serialize`.
/// - `middlewares`: A vector of middleware functions that can modify the request.
///
/// # Returns
/// A `Result` containing the deserialized response or a `FetchError` error.
pub async fn fetch<S, Q, B, R>(
    method: HttpMethod,
    url: &str,
    slugs: S,
    query: Q,
    body: B,
    middlewares: &Vec<Middleware>,
    cache: Rc<RefCell<dyn crate::cache::Cacheable>>,
) -> Result<R, FetchError>
where
    S: Serialize + Default + PartialEq,
    Q: Serialize + Default + PartialEq,
    B: Serialize + Default + PartialEq,
    R: for<'de> Deserialize<'de> + Default + 'static,
{
    let cache_policy = {
        let cache_ref = cache.borrow();
        cache_ref.policy().clone()
    };

    match cache_policy {
        CachePolicy::NetworkAndCache => {
            let cache_key = generate_cache_key(&method, url, &slugs, &query, &body)?;

            if let Some(cache_entry) = cache.borrow().get(&cache_key) {
                if cache_entry.timestamp > js_sys::Date::now() {
                    let result =
                        serde_json::from_value::<R>(cache_entry.data.clone()).map_err(|error| {
                            FetchError::ResponseDeserializationError(format!(
                                "{cache_entry:?} --- {error:?}"
                            ))
                        })?;

                    return Ok(result);
                }
            }

            let url = build_url(url, &slugs, query)?;
            let request = build_request(&url, &method, &body, middlewares)?;
            let response_text = send_request(&request).await?;

            if response_text.trim().is_empty() && TypeId::of::<R>() == TypeId::of::<String>() {
                return Ok(R::default());
            }

            let value = serde_json::from_str(&response_text).map_err(|error| {
                FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
            })?;

            cache.borrow_mut().set(&cache_key, &value);

            // Deserialize the response into the expected type
            let result = serde_json::from_value::<R>(value).map_err(|error| {
                FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
            })?;

            Ok(result)
        }
        CachePolicy::NetworkOnly => {
            let url = build_url(url, &slugs, query)?;
            let request = build_request(&url, &method, &body, middlewares)?;
            let response_text = send_request(&request).await?;

            if response_text.trim().is_empty() && TypeId::of::<R>() == TypeId::of::<String>() {
                return Ok(R::default());
            }

            let result = serde_json::from_str::<R>(&response_text).map_err(|error| {
                FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
            })?;

            Ok(result)
        }
        CachePolicy::CacheOnly => {
            let cache_key = generate_cache_key(&method, url, &slugs, &query, &body)?;

            if let Some(cache_entry) = cache.borrow().get(&cache_key) {
                if cache_entry.timestamp > js_sys::Date::now() {
                    let result =
                        serde_json::from_value::<R>(cache_entry.data.clone()).map_err(|error| {
                            FetchError::ResponseDeserializationError(format!(
                                "{cache_entry:?} --- {error:?}"
                            ))
                        })?;

                    return Ok(result);
                }
            }

            Ok(R::default())
        }
    }
}
