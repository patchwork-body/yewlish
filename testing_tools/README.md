# Yewlish Testing Tools

[![crates.io](https://img.shields.io/crates/v/yewlish-testing-tools.svg)](https://crates.io/crates/yewlish-testing-tools)
[![docs.rs](https://docs.rs/yewlish-testing-tools/badge.svg)](https://docs.rs/yewlish-testing-tools)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/yewlish-testing-tools.svg)](https://opensource.org/licenses/MIT)

**Yewlish Testing Tools** is a set of utilities designed to simplify testing components and hooks in the [Yew](https://yew.rs) framework. It provides a fluent API for querying, interacting with, and extracting information from rendered Yew components, making it easier to write and maintain tests for Yew applications.

## Features

- **Component Rendering**: Easily render Yew components for testing using the `render!` macro.
- **Hook Testing**: Test custom hooks with the `render_hook!` macro.
- **Querying**: Query elements by role, text, and custom test IDs.
- **Events**: Simulate user interactions such as clicks and key presses.
- **Attribute and Text Extraction**: Extract attributes and text content from elements.

## Installation

To use Yewlish Testing Tools in your project, add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
yewlish-testing-tools = "0.1.1"
```

## Prerequisites

Ensure that your project is set up to run tests in a browser environment, as this library is designed for testing web components.

## Usage

### Basic Example

Here’s a simple example of how to use Yewlish Testing Tools to render and test a Yew component:

```rust
use yew::prelude::*;
use yewlish_testing_tools::{render, Query};

#[derive(Properties, Clone, PartialEq)]
struct Props {
    text: &'static str,
}

#[function_component(TestComponent)]
fn test_component(props: &Props) -> Html {
    html! { <div>{ props.text }</div> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_render() {
        let t = render! {
            <TestComponent text="Hello, World!" />
        }
        .await;

        assert!(t.query_by_text("Hello, World!").exists());
    }
}
```

Hook Testing

You can test custom hooks using the render_hook! macro:

```rust
use yew::prelude::*;
use yewlish_testing_tools::render_hook;

#[hook]
fn use_counter() -> i32 {
use_state(|| 0)
}

#[cfg(test)]
mod tests {
use super::_;
use wasm_bindgen_test::_;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_use_counter() {
        let (hook, _) = render_hook!(i32, use_counter).await;
        assert_eq!(*hook.current(), 0);
    }

}

use yew::prelude::*;
use yewlish_testing_tools::render_hook;

#[hook]
fn use_counter() -> i32 {
use_state(|| 0)
}

#[cfg(test)]
mod tests {
    use yewlish_testing_tools::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_render_hook() {
        let (h, _) = render_hook!(UseStateHandle<bool>, {
            let a = use_state(|| true);
            a
        })
        .await;

        assert!(*h.get());
    }

    #[wasm_bindgen_test]
    async fn test_render_hook_with_effect() {
        let (h, _) = render_hook!(UseStateHandle<i32>, {
            let a = use_state(|| 0);

            {
                let a = a.clone();

                use_effect_with((), move |_| {
                    a.set(100);
                });
            }

            a
        })
        .await;

        assert_eq!(*h.get(), 100);
    }
}
```

## Documentation

The full documentation for Yewlish Testing Tools is available on [docs.rs](https://docs.rs/yewlish-testing-tools). It includes detailed examples and API references to help you get the most out of this library.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on [GitHub](https://github.com/patchwork-body/yewlish).

## License

Yewlish Testing Tools is dual-licensed under the MIT and Apache 2.0 licenses. You may choose to use either license at your option.

## Acknowledgements

This project is inspired by the need for robust testing tools in the Yew ecosystem and is influenced by the principles of the [testing-library](https://testing-library.com) from the JavaScript world. Special thanks to the Yew community for their continued support and contributions to the framework.

## Contact

For any questions or inquiries, feel free to reach out to the author:

**Kirill Korotkov**  
Email: [gugfug.personal@gmail.com](mailto:gugfug.personal@gmail.com)
