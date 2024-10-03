use html::IntoPropValue;
use std::default::Default;
use std::rc::Rc;
use yew::prelude::*;
use yewlish_attr_passer::*;
use yewlish_presence::*;
use yewlish_utils::hooks::{use_conditional_attr, use_controllable_state};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum CheckedState {
    Checked,
    #[default]
    Unchecked,
    Indeterminate,
}

impl IntoPropValue<Option<AttrValue>> for CheckedState {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            CheckedState::Checked => Some("checked".into()),
            CheckedState::Unchecked => Some("unchecked".into()),
            CheckedState::Indeterminate => Some("indeterminate".into()),
        }
    }
}

impl std::fmt::Display for CheckedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckedState::Checked => write!(f, "checked"),
            CheckedState::Unchecked => write!(f, "unchecked"),
            CheckedState::Indeterminate => write!(f, "indeterminate"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxContext {
    pub(crate) checked: CheckedState,
    pub(crate) disabled: bool,
}

pub enum CheckboxAction {
    Toggle,
}

impl Reducible for CheckboxContext {
    type Action = CheckboxAction;

    fn reduce(self: Rc<CheckboxContext>, action: Self::Action) -> Rc<CheckboxContext> {
        match action {
            CheckboxAction::Toggle => CheckboxContext {
                checked: match self.checked {
                    CheckedState::Checked => CheckedState::Unchecked,
                    CheckedState::Unchecked => CheckedState::Checked,
                    CheckedState::Indeterminate => CheckedState::Checked,
                },
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleCheckboxContext = UseReducerHandle<CheckboxContext>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxRenderAsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<CheckboxIndicator>,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub checked: CheckedState,
    #[prop_or_default]
    pub toggle: Callback<()>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub readonly: bool,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<CheckboxIndicator>,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<CheckedState>,
    #[prop_or_default]
    pub checked: Option<CheckedState>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<CheckedState>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub render_as: Option<Callback<CheckboxRenderAsProps, Html>>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let (checked, dispatch) = use_controllable_state(
        props.default_checked.clone(),
        props.checked.clone(),
        props.on_checked_change.clone(),
    );

    let context_value = use_reducer(|| CheckboxContext {
        checked: checked.borrow().clone(),
        disabled: props.disabled,
    });

    use_effect_with(
        ((*checked).clone().borrow().clone(), context_value.clone()),
        |(checked, context_value)| {
            if *checked != context_value.checked {
                context_value.dispatch(CheckboxAction::Toggle);
            }
        },
    );

    let toggle = use_callback(
        (dispatch.clone(), context_value.clone(), props.readonly),
        move |(), (dispatch, context_value, readonly)| {
            if *readonly {
                return;
            }

            dispatch.emit(Box::new(|prev_state| match prev_state {
                CheckedState::Checked => CheckedState::Unchecked,
                CheckedState::Unchecked => CheckedState::Checked,
                CheckedState::Indeterminate => CheckedState::Checked,
            }));

            context_value.dispatch(CheckboxAction::Toggle);
        },
    );

    let toggle_on_click = use_callback(toggle.clone(), move |_: MouseEvent, toggle| {
        toggle.emit(());
    });

    let prevent_checked_by_enter = use_callback((), |event: KeyboardEvent, _| {
        if event.key() == "Enter" {
            event.prevent_default();
        }
    });

    use_conditional_attr(props.r#ref.clone(), "data-disabled", props.disabled);

    let element = if let Some(render_as) = &props.render_as {
        html! {
            render_as.emit(CheckboxRenderAsProps {
                children: props.children.clone(),
                r#ref: props.r#ref.clone(),
                id: props.id.clone(),
                class: props.class.clone(),
                checked: checked.borrow().clone(),
                toggle: toggle.clone(),
                disabled: props.disabled,
                required: props.required,
                name: props.name.clone(),
                value: props.value.clone(),
                readonly: props.readonly,
            })
        }
    } else {
        html! {
            <AttrReceiver name="checkbox">
                <button
                    ref={props.r#ref.clone()}
                    id={props.id.clone()}
                    class={&props.class}
                    type="button"
                    role="checkbox"
                    disabled={props.disabled}
                    name={props.name.clone()}
                    value={props.value.clone()}
                    readonly={props.readonly}
                    onkeydown={prevent_checked_by_enter}
                    onclick={&toggle_on_click}
                >
                    {for props.children.iter()}
                </button>
            </AttrReceiver>
        }
    };

    html! {
        <ContextProvider<ReducibleCheckboxContext> context={context_value}>
            <AttrPasser name="checkbox" ..attributify! {
                "aria-checked" => match *checked.borrow() {
                    CheckedState::Checked => "true",
                    CheckedState::Unchecked => "false",
                    CheckedState::Indeterminate => "mixed",
                },
                "aria-required" => props.required.to_string(),
                "data-state" => checked.borrow().to_string(),
            }>
                {element}
            </AttrPasser>
        </ContextProvider<ReducibleCheckboxContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxIndicatorRenderAsProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub checked: CheckedState,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxIndicatorProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(CheckedState::Checked)]
    pub show_when: CheckedState,
    #[prop_or_default]
    pub render_as: Option<Callback<CheckboxIndicatorRenderAsProps, Html>>,
}

#[function_component(CheckboxIndicator)]
pub fn checkbox_indicator(props: &CheckboxIndicatorProps) -> Html {
    let context = use_context::<ReducibleCheckboxContext>()
        .expect("CheckboxIndicator must be a child of Checkbox");

    use_conditional_attr(props.r#ref.clone(), "data-disabled", context.disabled);

    let element = if let Some(render_as) = &props.render_as {
        html! {
            render_as.emit(CheckboxIndicatorRenderAsProps {
                r#ref: props.r#ref.clone(),
                class: props.class.clone(),
                children: props.children.clone(),
                checked: context.checked.clone(),
            })
        }
    } else {
        html! {
            <Presence
                name="checkbox-indicator"
                r#ref={props.r#ref.clone()}
                class={&props.class}
                present={context.checked == props.show_when}
                render_as={
                    Callback::from(|PresenceRenderAsProps { r#ref, class, presence, children }| {
                        html! {
                            <span
                                ref={r#ref.clone()}
                                class={&class}
                            >
                                { if presence {
                                    html! { {for children.iter()} }
                                } else {
                                    html! {}
                                } }
                            </span>
                        }
                    })
                }
            >
                {for props.children.iter()}
            </Presence>
        }
    };

    html! {
        <AttrPasser name="checkbox-indicator" ..attributify! {
            "data-state" => context.checked.to_string(),
        }>
            {element}
        </AttrPasser>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yewlish_testing_tools::TesterEvent;
    use yewlish_testing_tools::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_checkbox_should_toggle() {
        let t = render! {
            html! {
                <Checkbox>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        // The checkbox should be unchecked by default
        let checkbox = t.query_by_role("checkbox");
        assert!(checkbox.exists());

        assert_eq!(checkbox.attribute("disabled"), None);
        assert_eq!(checkbox.attribute("data-disabled"), None);

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "unchecked".to_string().into()
        );

        assert!(!t.query_by_text("X").exists());

        // After clicking, the state should be checked
        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "true".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "checked".to_string().into()
        );

        assert!(t.query_by_text("X").exists());

        // After clicking again, the state should be unchecked
        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "unchecked".to_string().into()
        );

        assert!(!t.query_by_text("X").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_default_checked() {
        let t = render! {
            html! {
                <Checkbox default_checked={CheckedState::Checked}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "true".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "checked".to_string().into()
        );

        assert!(t.query_by_text("X").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_default_unchecked() {
        let t = render! {
            html! {
                <Checkbox checked={CheckedState::Unchecked}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "unchecked".to_string().into()
        );

        assert!(!t.query_by_text("X").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_is_disabled() {
        let t = render! {
            html! {
                <Checkbox disabled={true}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert_eq!(
            checkbox.attribute("disabled"),
            "disabled".to_string().into()
        );

        assert_eq!(checkbox.attribute("data-disabled"), "".to_string().into());

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "unchecked".to_string().into()
        );

        assert!(!t.query_by_text("X").exists());

        // The checkbox should not toggle when disabled
        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("disabled"),
            "disabled".to_string().into()
        );

        assert_eq!(checkbox.attribute("data-disabled"), "".to_string().into());

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            checkbox.attribute("data-state"),
            "unchecked".to_string().into()
        );

        assert!(!t.query_by_text("X").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_attr_passer() {
        let t = render! {
            html! {
                <AttrPasser name="checkbox-indicator" ..attributify!{
                    "data-testid" => "checkbox-indicator-id",
                }>
                    <Checkbox>
                        <CheckboxIndicator></CheckboxIndicator>
                    </Checkbox>
                </AttrPasser>
            }
        }
        .await;

        assert!(t.query_by_testid("checkbox-indicator-id").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_accept_id() {
        let t = render! {
            html! {
                <Checkbox id={"id"}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");
        assert!(checkbox.exists());
        assert_eq!(checkbox.attribute("id"), "id".to_string().into());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_accept_class() {
        let t = render! {
            html! {
                <Checkbox class={"class"}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");
        assert!(checkbox.exists());
        assert_eq!(checkbox.attribute("class"), "class".to_string().into());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_is_required() {
        let t = render! {
            html! {
                <Checkbox required={true}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert!(checkbox.exists());
        assert_eq!(
            checkbox.attribute("aria-required"),
            "true".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_have_name() {
        let t = render! {
            html! {
                <Checkbox name={"name"}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");
        assert!(checkbox.exists());
        assert_eq!(checkbox.attribute("name"), "name".to_string().into());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_have_value() {
        let t = render! {
            html! {
                <Checkbox value={"value"}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert!(checkbox.exists());
        assert_eq!(checkbox.attribute("value"), "value".to_string().into());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_does_not_toggle_on_enter() {
        let t = render! {
            html! {
                <Checkbox>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert!(checkbox.exists());
        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        let checkbox = checkbox.keydown("Enter").await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_toggles_on_space() {
        let t = render! {
            html! {
                <Checkbox>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        }
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert!(checkbox.exists());
        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        let checkbox = checkbox.keydown(" ").await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_accept_ref() {
        let t = render!({
            let node_ref = use_node_ref();
            use_remember_value(node_ref.clone());

            html! {
                <Checkbox r#ref={node_ref}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        })
        .await;

        assert!(t.query_by_role("checkbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_on_checked_change() {
        let t = render!({
            let checked = use_state(|| CheckedState::Unchecked);

            let on_checked_change = use_callback((), {
                let checked = checked.clone();

                move |next_state: CheckedState, _| {
                    checked.set(next_state);
                }
            });

            use_remember_value(checked.clone());

            html! {
                <Checkbox checked={(*checked).clone()} on_checked_change={&on_checked_change}>
                    <CheckboxIndicator show_when={CheckedState::Checked}>{"X"}</CheckboxIndicator>
                </Checkbox>
            }
        })
        .await;

        let checkbox = t.query_by_role("checkbox");

        assert!(checkbox.exists());
        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            *t.get_state::<UseStateHandle<CheckedState>>(),
            CheckedState::Unchecked
        );

        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "true".to_string().into()
        );

        assert_eq!(
            *t.get_state::<UseStateHandle<CheckedState>>(),
            CheckedState::Checked
        );

        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            *t.get_state::<UseStateHandle<CheckedState>>(),
            CheckedState::Unchecked
        );
    }

    #[wasm_bindgen_test]
    async fn test_checkbox_render_as_input_checkbox() {
        let t = render!({
            let checked = use_state(|| CheckedState::Unchecked);

            let render_as = Callback::from(|props: CheckboxRenderAsProps| {
                let checked = props.checked == CheckedState::Checked;

                let onchange = {
                    let toggle = props.toggle.clone();

                    Callback::from(move |_event: Event| {
                        toggle.emit(());
                    })
                };

                html! {
                    <input
                        ref={props.r#ref.clone()}
                        id={props.id.clone()}
                        class={props.class.clone()}
                        type="checkbox"
                        checked={checked}
                        disabled={props.disabled}
                        required={props.required}
                        name={props.name.clone()}
                        aria-checked={if checked { "true" } else { "false" }}
                        value={props.value.clone()}
                        onchange={onchange}
                    />
                }
            });

            html! {
                <Checkbox
                    {render_as}
                    checked={(*checked).clone()}
                    on_checked_change={Callback::from(move |next_state| checked.set(next_state))}
                />
            }
        })
        .await;

        // The checkbox should be unchecked by default
        let checkbox = t.query_by_role("checkbox");
        assert!(checkbox.exists());

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(checkbox.attribute("disabled"), None);

        // After clicking, the checkbox should be checked
        let checkbox = checkbox.click().await;

        assert_eq!(
            checkbox.attribute("aria-checked"),
            "true".to_string().into()
        );

        // After clicking again, the checkbox should be unchecked
        let checkbox = checkbox.click().await;
        assert_eq!(
            checkbox.attribute("aria-checked"),
            "false".to_string().into()
        );
    }
}
