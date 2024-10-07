use serde::de::Deserialize;
use serde::Serialize;
use thiserror::Error;
use url::Url;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, Request, RequestInit, Response};

/// Enum representing HTTP methods.
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    fn as_str(&self) -> &str {
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
            "GET" => HttpMethod::GET,
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
    UrlParsingError(#[from] url::ParseError),
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

pub type Middleware = Box<dyn Fn(&mut RequestInit, &mut Headers)>;

/// Asynchronous function to perform an HTTP request using `web_sys` fetch API.
///
/// # Parameters
/// - `method`: The HTTP method to use for the request.
/// - `url`: The endpoint URL.
/// - `query`: Optional query parameters that implement `Serialize`.
/// - `body`: Optional request body that implements `Serialize`.
/// - `middlewares`: A vector of middleware functions that can modify the request.
///
/// # Returns
/// A `Result` containing the deserialized response or a `JsValue` error.
pub async fn fetch<S, Q, B, R>(
    method: HttpMethod,
    url: &str,
    slugs: S,
    query: Q,
    body: B,
    middlewares: Vec<Middleware>,
) -> Result<R, FetchError>
where
    S: Serialize + Default + PartialEq,
    Q: Serialize + Default + PartialEq,
    B: Serialize + Default + PartialEq,
    R: for<'de> Deserialize<'de>,
{
    // Build the URL with query parameters
    let url = {
        let mut url = url.to_string();

        // Serialize path parameters to a map
        let path_params_map = serde_json::to_value(&slugs)
            .map_err(|error| FetchError::SlugsSerializationError(error.to_string()))?;

        if let serde_json::Value::Object(map) = path_params_map {
            for (key, value) in map.iter() {
                let placeholder = format!("{{{}}}", key);
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };

                url = url.replace(&placeholder, &value_str);
            }
        }

        url
    };

    let mut url = Url::parse(&url).map_err(FetchError::UrlParsingError)?;

    if query != Q::default() {
        let query_string = serde_urlencoded::to_string(query)
            .map_err(|error| FetchError::QuerySerializationError(error.to_string()))?;

        url.set_query(Some(&query_string));
    }

    // Initialize the request
    let mut request_init = RequestInit::new();
    request_init.set_method(method.as_str());

    // Initialize headers
    let mut headers = Headers::new()
        .map_err(|error| FetchError::HeaderInitializationError(format!("{error:?}")))?;

    // Set the request body and headers
    if body != B::default() {
        let body_str = serde_json::to_string(&body)
            .map_err(|error| FetchError::BodySerializationError(error.to_string()))?;

        request_init.set_body(&JsValue::from_str(&body_str));

        headers
            .set("Content-Type", "application/json")
            .map_err(|error| FetchError::HeaderMutationError(format!("{error:?}")))?;
    }

    // Apply middlewares
    for middleware in middlewares {
        middleware(&mut request_init, &mut headers);
    }

    // Attach headers to the request
    request_init.set_headers(&headers);

    let request = Request::new_with_str_and_init(url.as_str(), &request_init)
        .map_err(|error| FetchError::HttpError(format!("Failed to create request: {error:?}")))?;

    // Perform the fetch operation
    let window = window().ok_or_else(FetchError::WindowNotFound)?;

    let response_js_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|error| FetchError::NetworkError(format!("Failed to fetch: {error:?}")))?;

    // Convert the response to a `Response` object
    let response: Response = response_js_value
        .dyn_into()
        .map_err(|error| FetchError::InvalidResponse(format!("Response error: {error:?}")))?;

    // Check if the response is OK (status in the range 200-299)
    if !response.ok() {
        return Err(FetchError::HttpError(format!(
            "Http Error: {}: {}",
            response.status(),
            response.status_text()
        )));
    }

    // Parse the response body
    let response_text = JsFuture::from(
        response
            .text()
            .map_err(|error| FetchError::JsonError(format!("{error:?}")))?,
    )
    .await
    .map_err(|error| FetchError::JsonError(format!("{error:?}")))?;

    // Deserialize the response into the expected type
    let result = serde_json::from_str::<R>(&response_text.as_string().unwrap_or_default())
        .map_err(|error| {
            FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
        })?;

    Ok(result)
}
