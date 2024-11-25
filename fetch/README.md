# yewlish-fetch

[![crates.io](https://img.shields.io/crates/v/yewlish-fetch.svg)](https://crates.io/crates/yewlish-fetch)
[![docs.rs](https://docs.rs/yewlish-fetch/badge.svg)](https://docs.rs/yewlish-fetch)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/yewlish-fetch.svg)](https://opensource.org/licenses/MIT)

**yewlish-fetch** is a Rust crate that provides a derive macro for generating Yew fetch API bindings. It simplifies the process of making HTTP requests in Yew applications by generating client code based on an enum definition.

## Features

- **Automatic Client Generation**: Define your API endpoints using an enum and derive the `FetchSchema` macro to generate client code.
- **Middleware Support**: Add middleware functions to modify requests before they are sent.
- **Error Handling**: Comprehensive error handling for various stages of the request lifecycle.
- **Async/Await**: Fully asynchronous API using Rust's async/await syntax.

## Installation

To use yewlish-fetch in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
yewlish-fetch = "0.3.0"

# required peer-dependencies
yewlish-fetch-utils = "0.3.0"
```

in case you want to use web sockets, make sure you have `web-sys` with `WebSocket` feature enabled.

```toml
web-sys = {version = "0.3.72", features = ["AbortController", "WebSocket"]}
```

## Prerequisites

Ensure that your project is set up to run in a browser environment, as this library is designed for web applications using Yew.

## Quick Start

### Basic Example

Here’s a simple example of how to use yewlish-fetch to define an API schema and make a request:

```rust
use yew::prelude::*;
use yewlish_fetch::{FetchSchema, FetchService, FetchError, FetchResponse};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, PartialEq, Clone)]
struct PostSlugs {
    id: u32,
}

#[derive(Default, Serialize, PartialEq, Clone)]
struct GetPostCommentsQuery {
    id: u32,
}

#[derive(Default, Deserialize, Debug, Serialize, PartialEq, Clone)]
struct PostBody {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
struct CommentBody {
    id: u32,
    name: String,
    email: String,
    body: String,
    #[serde(rename = "postId")]
    post_id: u32,
}

// Define a schema for your API by creating an enum and deriving FetchSchema.
#[derive(FetchSchema)]
enum Api {
    // Define your API endpoints using the #[get], #[post], #[put], #[patch], or #[delete] attributes.
    // The `res` attribute specifies the response type for the endpoint.
    #[get("/posts", res = Vec<PostBody>)]
    // Each variant will be used to generate a client function for making requests.
    GetPosts,
    // You can also define path parameters using the `slugs` attribute.
    #[get("/posts/{id}", slugs = PostSlugs, res = PostBody)]
    GetPost,
    #[get("/posts/{id}/comments", slugs = PostSlugs, res = Vec<CommentBody>)]
    GetPostComments,
    // Define query parameters using the `query` attribute.
    #[get("/comments", query = GetPostCommentsQuery, res = Vec<CommentBody>)]
    GetComments,
    // Define request body parameters using the `body` attribute.
    #[post("/posts", body = PostBody, res = PostBody)]
    CreatePost,
    #[put("/posts/{id}", slugs = PostSlugs, body = PostBody, res = PostBody)]
    UpdatePost,
    #[patch("/posts/{id}", slugs = PostSlugs, body = PostBody, res = PostBody)]
    PatchPost,
    #[delete("/posts/{id}", slugs = PostSlugs)]
    DeletePost,
}

#[derive(Properties, Clone, PartialEq)]
struct AppWrapperProps {
    children: Children,
}

#[function_component(AppWrapper)]
fn app_wrapper(props: &AppWrapperProps) -> Html {
    // Create a new ApiFetchClient instance with the base URL of your API.
    let client =
        ApiFetchClient::new("https://jsonplaceholder.typicode.com").with_middlewares(
            // Add middleware functions to modify requests before they are sent.
            vec![
                Rc::new(|_request_init, headers| {
                    let headers = headers.clone();

                    // Of course this is a dummy middleware, but it's just for testing purposes
                    let future = async move {
                        headers
                            .borrow_mut()
                            .set("Authorization", "Bearer token")
                            .unwrap();
                    };

                    Box::pin(future)
                }),
            ]
        );

    // ApiFetchClientProvider is a context provider that allows you to access the client instance in child components.
    html! {
        <ApiFetchClientProvider client={client}>
            {for props.children.iter()}
        </ApiFetchClientProvider>
    }
}

#[function_component(GetPosts)]
fn get_posts() -> Html {
    // `Api::GetPosts` variant definition will generate four hook versions:
    // - `use_get_posts`
    // - `use_get_posts_async`
    // - `use_get_posts_with_options`
    // - `use_get_posts_with_options_async`
    let posts = use_get_posts_async();

    // `posts` is a `UseGetPostsAsyncHandle` (generated struct) instance that contains
    // - the response `data: UseStateHandle<Option<#res>>` (deserialized json according to defined `res = Vec<PostBody>`)
    // - error `error: UseStateHandle<Option<FetchError>>`
    // - loading state `loading: UseStateHandle<bool>`
    // - optional trigger (only when the async version of the hook is used) `trigger: Callback<#method_params_struct_name>` (in this case `Callback<GetPostsParams>`)

    if let Some(error) = (*posts.error).clone() {
        return html! { format!("Error fetching posts: {error:?}") }
    }

    html! {
        <>
            <button onclick={Callback::from(move |_event: MouseEvent| {
                posts.trigger.emit(GetPostsParams::default());
            })}>{ "Fetch" }</button>

            {if *posts.loading {
                html! { "Loading..." }
            } else {
                html! {
                    <ul>
                        {for (*posts.data).clone().unwrap_or_default().iter().map(|post| html! {
                            <li key={post.id}>
                                <p>{&post.title}</p>
                                <p>{&post.body}</p>
                            </li>
                        })}
                    </ul>
                }
            }}
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <AppWrapper>
            <GetPosts />
        </AppWrapper>
    }
}
```

## Architecture

Yewlish Fetch is built on several key components:

### Core Components

1. **FetchSchema Derive Macro**
   - Generates type-safe API client code from enum definitions
   - Handles URL parameter substitution and query string building
   - Creates strongly-typed request/response handling

2. **Client Layer**
   - Manages HTTP requests and WebSocket connections
   - Handles request lifecycle (preparation, execution, response handling)
   - Provides middleware support for request/response transformation

3. **Cache System**
   - Implements various caching strategies:
     - StaleWhileRevalidate: Update cache while serving stale data
     - CacheThenNetwork: Serve from cache first, then network (only if cache is expired)
     - NetworkOnly: Always fetch from network
     - CacheOnly: Serve only from cache
   - Automatic cache invalidation
   - Configurable cache duration

4. **State Management**
   - Hooks for managing request state
   - Automatic state synchronization across components based on signals
   - Built-in loading and error states

## Documentation

The full documentation for Yewlish Fetch is available on [docs.rs](https://docs.rs/yewlish-fetch). It includes detailed examples and API references to help you get the most out of this library.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on [GitHub](https://github.com/patchwork-body/yewlish).

## License

Yewlish Fetch is dual-licensed under the MIT and Apache 2.0 licenses. You may choose to use either license at your option.

## Acknowledgements

This project is inspired by the need for robust fetch utilities in the Yew ecosystem. Special thanks to the Yew community for their continued support and contributions to the framework.

## Contact

For any questions or inquiries, feel free to reach out to the author:

**Kirill Korotkov**
Email: [personal.gugfug@gmail.com](mailto:personal.gugfug@gmail.com)
