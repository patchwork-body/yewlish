# Yewlish Testing Tools

[![crates.io](https://img.shields.io/crates/v/yewlish-testing-tools.svg)](https://crates.io/crates/yewlish-testing-tools)
[![docs.rs](https://docs.rs/yewlish-testing-tools/badge.svg)](https://docs.rs/yewlish-testing-tools)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/yewlish-testing-tools.svg)](https://opensource.org/licenses/MIT)

**Yewlish Testing Tools** is a set of utilities designed to simplify testing components and hooks in the [Yew](https://yew.rs) framework. It provides a fluent API for querying, interacting with, and extracting information from rendered Yew components, making it easier to write and maintain tests for Yew applications.

## Features

- **Component Rendering**: Easily render Yew components or custom hooks with using the `render!` macro.
- **Querying**: Query elements by role, text, and custom test IDs.
- **Events**: Simulate user interactions such as clicks and key presses.
- **Attribute and Text Extraction**: Extract attributes and text content from elements.

## Installation

To use Yewlish Testing Tools in your project, add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
yewlish-testing-tools = "1.1.2"
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
        let t = render!({
            html! {
                <TestComponent text="Hello, World!" />
            }
        })
        .await;

        assert!(t.query_by_text("Hello, World!").exists());
    }
}
```

### Component Testing

You can test Yew components using the render! macro:

```rust
#[cfg(test)]
mod tests {
    use crate::{render, Query, TesterEvent};
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_render() {
        let t = render!({
            let counter = use_state(|| 0);

            let increment = use_callback(counter.clone(), |_event: MouseEvent, counter| {
                counter.set(**counter + 1);
            });

            use_remember_value(counter.clone());

            html! {
                <button onclick={&increment}>{"Click me "}{*counter}</button>
            }
        })
        .await;

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 0);

        let button = t.query_by_role("button");
        assert!(button.exists());
        assert!(button.text().contains("Click me 0"));

        let button = button.click().await;
        assert!(button.text().contains("Click me 1"));

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 1);
    }
}
```

### Hook Testing

You can test custom hooks using the render_hook! macro:

```rust
#[cfg(test)]
mod tests {
    use crate::{render, Extractor, Query, TesterEvent};
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_render_with_state() {
        let t = render!({
            let state = use_state(|| true);
            use_remember_value(state);

            html! {}
        })
        .await;

        assert!(*t.get_state::<UseStateHandle<bool>>());
    }

    #[wasm_bindgen_test]
    async fn test_render_with_effect() {
        let t = render!({
            let state = use_state(|| 0);

            {
                let state = state.clone();

                use_effect_with((), move |_| {
                    state.set(100);
                });
            }

            use_remember_value(state.clone());

            html! {}
        })
        .await;

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 100);
    }

    #[wasm_bindgen_test]
    async fn test_render_with_reducer() {
        #[derive(Clone, PartialEq)]
        struct Counter {
            count: i32,
        }

        enum CounterAction {
            Increment,
            Decrement,
        }

        impl Reducible for Counter {
            type Action = CounterAction;

            fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
                match action {
                    CounterAction::Increment => Self {
                        count: self.count + 1,
                    }
                    .into(),
                    CounterAction::Decrement => Self {
                        count: self.count - 1,
                    }
                    .into(),
                }
            }
        }

        let t = render!({
            let state = use_reducer(|| Counter { count: 0 });
            use_remember_value(state.clone());

            html! {}
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 0);

        t.act(|| {
            t.get_state::<UseReducerHandle<Counter>>()
                .dispatch(CounterAction::Increment);
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 1);

        t.act(|| {
            t.get_state::<UseReducerHandle<Counter>>()
                .dispatch(CounterAction::Decrement);
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 0);
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
Email: [personal.gugfug@gmail.com](mailto:personal.gugfug@gmail.com)
