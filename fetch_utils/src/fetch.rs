use crate::{
    helpers::{build_request, build_url, send_request},
    FetchError, Middleware,
};
use serde::Serialize;
use std::rc::Rc;

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

pub struct FetchOptions<'a, S, Q, B> {
    pub slugs: S,
    pub query: Q,
    pub body: B,
    pub middlewares: &'a [Middleware],
    pub abort_signal: Rc<web_sys::AbortSignal>,
}

#[allow(clippy::too_many_lines)]
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
pub async fn fetch<'a, S, Q, B>(
    method: HttpMethod,
    url: &str,
    options: FetchOptions<'a, S, Q, B>,
) -> Result<String, FetchError>
where
    S: Serialize + Default + PartialEq,
    Q: Serialize + Default + PartialEq,
    B: Serialize + Default + PartialEq,
{
    let url = build_url(url, &options.slugs, &options.query)?;

    let request = build_request(
        &url,
        &method,
        &options.body,
        options.middlewares,
        &options.abort_signal,
    )
    .await?;

    let response_text = send_request(&request).await?;

    Ok(response_text)
}
