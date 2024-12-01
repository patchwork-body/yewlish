# Yewlish Checkbox

[![crates.io](https://img.shields.io/crates/v/yewlish-checkbox.svg)](https://crates.io/crates/yewlish-checkbox)
[![docs.rs](https://docs.rs/yewlish-checkbox/badge.svg)](https://docs.rs/yewlish-checkbox)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/yewlish-checkbox.svg)](https://opensource.org/licenses/MIT)

**Yewlish Checkbox** is a customizable checkbox component for the [Yew](https://yew.rs) framework. It offers a flexible API to create accessible and interactive checkbox inputs, allowing developers to manage various states and behaviors seamlessly within their Yew applications.

## Features

- **Multiple States**: Supports `checked`, `unchecked`, and `indeterminate` states.
- **Accessibility**: Implements ARIA attributes to ensure compliance with accessibility standards.
- **Custom Rendering**: Allows rendering as different HTML elements if needed.
- **Event Handling**: Provides callbacks for state changes and interactions.
- **Theming**: Easily styled via CSS classes to match your application's design.

## Installation

Add the crate to your Cargo.toml:

```toml
[dependencies]
yew = "^0.21"
yewlish-checkbox = "^0.1"
```

## Usage

### Basic Checkbox

Create a simple checkbox with default props.

```rust
use yew::prelude::*;
use yewlish_checkbox::{Checkbox, CheckedState};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Checkbox>
          <CheckboxIndicator
            shows_when={CheckboxState::Checked}
          >
            {"✔"}
          </CheckboxIndicator>
        </Checkbox>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

- The Checkbox component handles its own state internally.
- The CheckboxIndicator displays the check mark "✔" when the checkbox is in the CheckedState::Checked state.

### Controlled Checkbox

Manage the checkbox state externally using the checked and on_checked_change props.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    let checked_state = use_state(|| CheckedState::Unchecked);

    let on_checked_change = {
        let checked_state = checked_state.clone();
        Callback::from(move |new_state: CheckedState| {
            checked_state.set(new_state);
        })
    };

    html! {
        <Checkbox
            checked={(*checked_state).clone()}
            on_checked_change={on_checked_change}
        >
            <CheckboxIndicator show_when={CheckedState::Checked}>
                {"✔"}
            </CheckboxIndicator>
        </Checkbox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- The checkbox state is stored in a UseStateHandle.
- The on_checked_change callback updates the state when the checkbox is toggled.

### Indeterminate State

Use the indeterminate state for partially selected options.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Checkbox default_checked={CheckboxState::Indeterminate}>
            <CheckboxIndicator show_when={CheckedState::Checked}>
                {"✔"}
            </CheckboxIndicator>
            <CheckboxIndicator show_when={CheckedState::Indeterminate}>
                {"−"}
            </CheckboxIndicator>
        </Checkbox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- Displays a minus sign "−" when in the Indeterminate state.

### Custom Rendering

Customize how the checkbox is rendered using the render_as prop.

```rust
use yew::prelude::*;
use yewlish_checkbox::{Checkbox, CheckboxRenderAsProps, CheckedState};

#[function_component(App)]
fn app() -> Html {
    let render_as = Callback::from(|props: CheckboxRenderAsProps| {
        let is_checked = props.checked == CheckedState::Checked;

        html! {
            <label class={props.class.clone()}>
                <input
                    type="checkbox"
                    checked={is_checked}
                    onclick={props.toggle.clone()}
                    disabled={props.disabled}
                    required={props.required}
                    name={props.name.clone()}
                    value={props.value.clone()}
                />

                { for props.children.iter() }
            </label>
        }
    });

    html! {
        <Checkbox render_as={render_as}>
            { "Custom Checkbox" }
        </Checkbox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- The checkbox is rendered as a standard HTML checkbox input.
- You can customize the HTML structure and styling.

### Styling

Apply custom styles using the class prop.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Checkbox class="custom-checkbox">
            <CheckboxIndicator
              show_when={CheckedState::Checked}
              class="custom-indicator"
            >
                {"✔"}
            </CheckboxIndicator>
        </Checkbox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

Define styles in your CSS:

```css
.custom-checkbox {
    display: flex;
    align-items: center;
}

.custom-indicator {
    color: green;
}
```

### Disabled and Read-Only Checkboxes

Disable interaction using the disabled or readonly props.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Checkbox disabled={true}>
                <CheckboxIndicator show_when={CheckedState::Checked}>
                    {"Disabled Checkbox"}
                </CheckboxIndicator>
            </Checkbox>
            <Checkbox default_checked={CheckedState::Checked} readonly={true}>
                <CheckboxIndicator show_when={CheckedState::Checked}>
                    {"Read-Only Checkbox"}
                </CheckboxIndicator>
            </Checkbox>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- A disabled checkbox cannot be interacted with and appears disabled.
- A read-only checkbox displays its state but cannot be changed by the user.

### Using with Forms

Integrate with forms using the name and value props.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <form>
            <label for={"terms_agreement"}>
              {"I agree to the terms"}
            </label>

            <Checkbox id="terms_agreement" name="agree_terms" value="yes">
                <CheckboxIndicator show_when={CheckedState::Checked}>
                  {"✔"}
                </CheckboxIndicator>
            </Checkbox>

            <button type="submit">{"Submit"}</button>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### Handling State Changes

Perform actions when the checkbox state changes.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;

#[function_component(App)]
fn app() -> Html {
    let on_checked_change = Callback::from(|new_state: CheckedState| {
        if new_state == CheckedState::Checked {
          log::debug!("Checkbox is checked");
        }
    });

    html! {
        // You don't need to provide a value,
        // this checkbox will still be uncontrollable
        <Checkbox on_checked_change={on_checked_change}>
            <CheckboxIndicator show_when={CheckedState::Checked}>
                {"✔"}
            </CheckboxIndicator>
        </Checkbox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### Accessibility

The yewlish-checkbox component is designed to satisfy the [W3C ARIA accessibility pattern guide for checkboxes](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/). It includes ARIA attributes to ensure it is accessible to all users. The `role` attribute is set to `checkbox`, and the `aria-checked` attribute reflects the current state ("true", "false", or "mixed").

To ensure full accessibility:

- Use `CheckboxIndicator` to provide visual feedback.
- Set the `disabled` and `required` props as needed.
- Provide appropriate labels and descriptions.

```rust
use yew::prelude::*;
use yewlish_checkbox::*;
use yewlish_attr_passer::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <AttrPasser name="checkbox" ..attributify {
          "aria-label" => "Check me"
        }>
          <Checkbox on_checked_change={on_checked_change}>
              <CheckboxIndicator show_when={CheckedState::Checked}>
                {"✔"}
              </CheckboxIndicator>
          </Checkbox>
        </AttrPasser>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Checkbox Properties

| Property           | Type                                           | Default     | Description                                      |
|--------------------|------------------------------------------------|-------------|--------------------------------------------------|
| children           | ChildrenWithProps<CheckboxIndicator>           | None        | Children elements, typically one or more CheckboxIndicators. |
| ref                | NodeRef                                        | None        | Reference to the checkbox DOM node.              |
| id                 | Option<AttrValue>                              | None        | The ID of the checkbox element.                  |
| class              | Option<AttrValue>                              | None        | CSS class for styling the checkbox.              |
| default_checked    | Option<CheckedState>                           | Unchecked   | The default checked state.                       |
| checked            | Option<CheckedState>                           | None        | Controlled checked state.                        |
| disabled           | bool                                           | false       | Whether the checkbox is disabled.                |
| on_checked_change  | Callback<CheckedState>                         | None        | Callback when the checked state changes.         |
| required           | bool                                           | false       | Whether the checkbox is required.                |
| name               | Option<AttrValue>                              | None        | The name attribute of the checkbox.              |
| value              | Option<AttrValue>                              | None        | The value attribute of the checkbox.             |
| readonly           | bool                                           | false       | Whether the checkbox is read-only.               |
| render_as          | Option<Callback<CheckboxRenderAsProps, Html>>  | None        | Custom render function for the checkbox.         |

## CheckboxIndicator Properties

| Property   | Type                                           | Default | Description                                      |
|------------|------------------------------------------------|---------|--------------------------------------------------|
| ref        | NodeRef                                        | None    | Reference to the indicator DOM node.             |
| class      | Option<AttrValue>                              | None    | CSS class for styling the indicator.             |
| children   | Children                                       | None    | Children elements to display inside the indicator.|
| show_when  | CheckedState                                   | Checked | The state when the indicator is visible.         |
| render_as  | Option<Callback<CheckboxIndicatorRenderAsProps, Html>> | None    | Custom render function for the indicator.        |

### Testing

The yewlish-checkbox crate includes a suite of tests to ensure reliability. Tests can be run using:

```sh
wasm-pack test --firefox --headless
```

Ensure you have wasm-pack and a compatible browser installed.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on [GitHub](https://github.com/patchwork-body/yewlish).

## License

Yewlish Checkbox is dual-licensed under the MIT and Apache 2.0 licenses. You may choose to use either license at your option.

## Acknowledgements

This project is inspired by the [Radix UI](https://www.radix-ui.com/primitives/docs/components/checkbox) library, known for its robust and accessible UI components. Special thanks to the Radix community for their excellent work and contributions, which have greatly influenced the development of Yewlish Checkbox.

## Contact

For any questions or inquiries, feel free to reach out to the author:

**Kirill Korotkov**
Email: [personal.gugfug@gmail.com](mailto:personal.gugfug@gmail.com)
