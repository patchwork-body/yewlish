use crate::{Cacheable, FetchError, HttpMethod, Middleware};
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::{any::TypeId, cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, Request, RequestInit, Response, Url};

pub fn generate_cache_key<S, Q, B>(
    method: &HttpMethod,
    url: &str,
    slugs: &S,
    query: &Q,
    body: &B,
) -> Result<String, FetchError>
where
    S: Serialize,
    Q: Serialize,
    B: Serialize,
{
    let mut hasher = Sha1::new();
    hasher.update(method.as_str().as_bytes());
    hasher.update(url.as_bytes());

    let slugs_bytes = serde_json::to_vec(slugs)
        .map_err(|error| FetchError::SlugsSerializationError(error.to_string()))?;
    hasher.update(&slugs_bytes);

    let query_bytes = serde_json::to_vec(query)
        .map_err(|error| FetchError::QuerySerializationError(error.to_string()))?;
    hasher.update(&query_bytes);

    let body_bytes = serde_json::to_vec(body)
        .map_err(|error| FetchError::BodySerializationError(error.to_string()))?;
    hasher.update(&body_bytes);

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn build_url<S, Q>(url: &str, slugs: &S, query: Q) -> Result<Url, FetchError>
where
    S: Serialize + Default + PartialEq,
    Q: Serialize + Default + PartialEq,
{
    let url = {
        let mut url = url.to_string();

        // Serialize path parameters to a map
        let path_params_map = serde_json::to_value(slugs)
            .map_err(|error| FetchError::SlugsSerializationError(error.to_string()))?;

        if let serde_json::Value::Object(map) = path_params_map {
            for (key, value) in &map {
                let placeholder = format!("{{{key}}}");

                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };

                url = url.replace(&placeholder, &value_str);
            }
        }

        url
    };

    let url = Url::new(&url).map_err(|_| FetchError::UrlParsingError("Invalid URL".to_string()))?;

    if query != Q::default() {
        let query_string = serde_urlencoded::to_string(query)
            .map_err(|error| FetchError::QuerySerializationError(error.to_string()))?;

        url.set_search(&query_string);
    }

    Ok(url)
}

pub fn build_request<T>(
    url: &Url,
    method: &HttpMethod,
    body: &T,
    middlewares: &Vec<Middleware>,
) -> Result<Request, FetchError>
where
    T: Serialize + Default + PartialEq,
{
    let mut request_init = RequestInit::new();
    request_init.set_method(method.as_str());

    // Initialize headers
    let mut headers = Headers::new()
        .map_err(|error| FetchError::HeaderInitializationError(format!("{error:?}")))?;

    // Set the request body and headers
    if *body != T::default() {
        let body_str = serde_json::to_string(&body)
            .map_err(|error| FetchError::BodySerializationError(error.to_string()))?;

        request_init.set_body(&JsValue::from_str(&body_str));

        headers
            .set("Content-Type", "application/json")
            .map_err(|error| FetchError::HeaderMutationError(format!("{error:?}")))?;
    }

    // Apply middlewares
    for middleware in middlewares {
        let middleware = Arc::clone(middleware);
        middleware(&mut request_init, &mut headers);
    }

    // Attach headers to the request
    request_init.set_headers(&headers);

    let request =
        Request::new_with_str_and_init(String::from(url.to_string()).as_str(), &request_init)
            .map_err(|error| {
                FetchError::HttpError(format!("Failed to create request: {error:?}"))
            })?;

    Ok(request)
}

pub async fn send_request(request: &Request) -> Result<String, FetchError> {
    // Perform the fetch operation
    let window = window().ok_or_else(FetchError::WindowNotFound)?;

    let response_js_value = JsFuture::from(window.fetch_with_request(request))
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

    Ok(response_text.as_string().unwrap_or_default())
}

pub fn deserialize_cached_data<R>(data: &serde_json::Value) -> Result<R, FetchError>
where
    R: for<'de> serde::Deserialize<'de>,
{
    let result = serde_json::from_value::<R>(data.clone()).map_err(|error| {
        FetchError::ResponseDeserializationError(format!("{data:?} --- {error:?}"))
    })?;

    Ok(result)
}

pub fn deserialize_response<R>(response_text: &str) -> Result<R, FetchError>
where
    R: for<'de> serde::Deserialize<'de> + Default + 'static,
{
    if response_text.trim().is_empty()
        && std::any::TypeId::of::<R>() == std::any::TypeId::of::<String>()
    {
        return Ok(R::default());
    }

    let value = serde_json::from_str(response_text).map_err(|error| {
        FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
    })?;

    Ok(value)
}

pub fn deserialize_response_and_store_cache<R>(
    response_text: &str,
    cache: &Rc<RefCell<dyn Cacheable>>,
    cache_key: &str,
    max_age: Option<f64>,
) -> Result<R, FetchError>
where
    R: for<'de> serde::Deserialize<'de> + Default + 'static,
{
    if response_text.trim().is_empty() && TypeId::of::<R>() == TypeId::of::<String>() {
        return Ok(R::default());
    }

    let value = serde_json::from_str(response_text).map_err(|error| {
        FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
    })?;

    cache.borrow_mut().set(cache_key, &value, max_age);

    // Deserialize the response into the expected type
    let result = serde_json::from_value::<R>(value).map_err(|error| {
        FetchError::ResponseDeserializationError(format!("{response_text:?} --- {error:?}"))
    })?;

    Ok(result)
}
