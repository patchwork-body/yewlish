use std::rc::Rc;

use yew::prelude::*;
use yewlish_attr_passer::{attributify, AttrPasser, AttrReceiver};
use yewlish_roving_focus::*;
use yewlish_utils::{
    enums::{DataState, Dir, Orientation},
    hooks::use_conditional_attr,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupContext {
    pub name: Option<AttrValue>,
    pub value: AttrValue,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
}

pub enum RadioGroupAction {
    SetValue(AttrValue),
}

impl Reducible for RadioGroupContext {
    type Action = RadioGroupAction;

    fn reduce(self: Rc<RadioGroupContext>, action: Self::Action) -> Rc<RadioGroupContext> {
        match action {
            RadioGroupAction::SetValue(value) => RadioGroupContext {
                value,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleRadioGroupContext = UseReducerHandle<RadioGroupContext>;

#[derive(Debug, Clone, PartialEq, Properties)]
#[allow(clippy::struct_excessive_bools)]
pub struct RadioGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub dir: Option<Dir>,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or(true)]
    pub r#loop: bool,
    #[prop_or_default]
    pub on_value_change: Callback<AttrValue>,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let context_value = use_reducer(|| RadioGroupContext {
        name: props.name.clone(),
        value: props
            .value
            .clone()
            .unwrap_or(props.default_value.clone().unwrap_or_default()),
        required: props.required,
        readonly: props.readonly,
        disabled: props.disabled,
    });

    use_effect_with(
        (context_value.value.clone(), props.on_value_change.clone()),
        |(value, on_value_change)| {
            on_value_change.emit(value.clone());
        },
    );

    html! {
        <ContextProvider<ReducibleRadioGroupContext> context={context_value}>
            <RovingFocus
                role="radiogroup"
                class={&props.class}
                orientation={props.orientation.clone()}
                dir={props.dir.clone().unwrap_or(Dir::Ltr)}
                r#loop={props.r#loop}
            >
                {for props.children.iter()}
            </RovingFocus>
        </ContextProvider<ReducibleRadioGroupContext>>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupItemContext {
    pub checked: bool,
}

pub enum RadioGroupItemAction {
    Check,
    Uncheck,
}

impl Reducible for RadioGroupItemContext {
    type Action = RadioGroupItemAction;

    fn reduce(self: Rc<RadioGroupItemContext>, action: Self::Action) -> Rc<RadioGroupItemContext> {
        match action {
            RadioGroupItemAction::Check => RadioGroupItemContext { checked: true }.into(),
            RadioGroupItemAction::Uncheck => RadioGroupItemContext { checked: false }.into(),
        }
    }
}

type ReducibleRadioGroupItemContext = UseReducerHandle<RadioGroupItemContext>;

#[derive(Clone, Debug, PartialEq, Properties)]
#[allow(clippy::struct_excessive_bools)]
pub struct RadioGroupItemRenderAsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<RadioGroupItemIndicator>,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub toggle: Callback<()>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RadioGroupItemProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<RadioGroupItemIndicator>,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub render_as: Option<Callback<RadioGroupItemRenderAsProps, Html>>,
}

#[function_component(RadioGroupItem)]
pub fn radio_group_item(props: &RadioGroupItemProps) -> Html {
    let group_context = use_context::<ReducibleRadioGroupContext>()
        .expect("RadioGroupItem must be a child of RadioGroup");

    let checked = use_memo(
        (group_context.clone(), props.value.clone(), props.checked),
        |(group_context, props_value, props_checked)| {
            if let Some(props_checked) = props_checked {
                return *props_checked;
            }

            group_context.value == props_value.clone().unwrap_or_default()
        },
    );

    let context_value = use_reducer(|| RadioGroupItemContext { checked: *checked });

    use_effect_with(
        (checked.clone(), context_value.clone()),
        |(checked, context_value)| {
            if **checked {
                context_value.dispatch(RadioGroupItemAction::Check);
            } else {
                context_value.dispatch(RadioGroupItemAction::Uncheck);
            }
        },
    );

    let toggle = use_callback(
        (
            group_context.clone(),
            props.value.clone(),
            props.checked,
            group_context.readonly,
        ),
        move |(), (group_context, value, checked, readonly)| {
            if *readonly {
                return;
            }

            if checked.unwrap_or_default() {
                return;
            }

            group_context.dispatch(RadioGroupAction::SetValue(
                value.clone().unwrap_or_default(),
            ));
        },
    );

    let toggle_by_click = use_callback(toggle.clone(), move |_: MouseEvent, toggle| {
        toggle.emit(());
    });

    let toggle_by_focus = use_callback(toggle.clone(), move |_: FocusEvent, toggle| {
        toggle.emit(());
    });

    let disabled = props.disabled || group_context.disabled;

    use_conditional_attr(props.r#ref.clone(), "data-disabled", None, disabled);

    let element = if let Some(render_as) = &props.render_as {
        render_as.emit(RadioGroupItemRenderAsProps {
            children: props.children.clone(),
            r#ref: props.r#ref.clone(),
            id: props.id.clone(),
            class: props.class.clone(),
            checked: *checked,
            toggle: toggle.clone(),
            disabled,
            required: group_context.required,
            name: props.name.clone(),
            value: props.value.clone(),
            readonly: group_context.readonly,
        })
    } else {
        html! {
            <AttrReceiver name="radio-group-item">
                <button
                    ref={props.r#ref.clone()}
                    role="radio"
                    type="button"
                    disabled={disabled}
                    id={props.id.clone()}
                    class={&props.class}
                    name={props.name.clone().unwrap_or_else(|| group_context.name.clone().unwrap_or_default())}
                    value={props.value.clone()}
                    onclick={&toggle_by_click}
                    onfocus={&toggle_by_focus}
                >
                    {for props.children.iter()}
                </button>
            </AttrReceiver>
        }
    };

    html! {
        <ContextProvider<ReducibleRadioGroupItemContext> context={context_value}>
            <AttrPasser name="radio-group-item" ..attributify! {
                "data-state" => if *checked { DataState::On } else { DataState::Off },
                "aria-checked" => checked.to_string(),
            }>
                {element}
            </AttrPasser>
        </ContextProvider<ReducibleRadioGroupItemContext>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RadioGroupItemIndicatorProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RadioGroupItemIndicator)]
pub fn radio_group_item_indicator(props: &RadioGroupItemIndicatorProps) -> Html {
    let context = use_context::<ReducibleRadioGroupItemContext>()
        .expect("RadioGroupItemIndicator must be a child of RadioGroupItem");

    if !context.checked {
        return html! {};
    }

    html! {
        <span id={&props.id} class={&props.class}>
            {for props.children.iter()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yewlish_testing_tools::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_radio_group_should_change_value_on_item_click() {
        let t = render!({
            let selected_value = use_state(|| AttrValue::from("item2"));

            let on_value_change = use_callback((), {
                let selected_value = selected_value.clone();

                move |value: AttrValue, ()| {
                    selected_value.set(value);
                }
            });

            use_remember_value(selected_value.clone());

            html! {
                <RadioGroup value={(*selected_value).clone()} on_value_change={&on_value_change}>
                    <div>
                        <RadioGroupItem id="radio#1" name="radio" value="item1">
                            <RadioGroupItemIndicator />
                        </RadioGroupItem>

                        <label for="radio#1" class="text-neutral-200">{"Option 1"}</label>
                    </div>

                    <div>
                        <RadioGroupItem id="radio#2" name="radio" value="item2">
                            <RadioGroupItemIndicator />
                        </RadioGroupItem>

                        <label for="radio#2" class="text-neutral-200">{"Option 2"}</label>
                    </div>

                    <div>
                        <RadioGroupItem id="radio#3" name="radio" value="item3">
                            <RadioGroupItemIndicator />
                        </RadioGroupItem>

                        <label for="radio#3" class="text-neutral-200">{"Option 3"}</label>
                    </div>
                </RadioGroup>

            }
        })
        .await;

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 3);

        // Initially, the second item should be checked
        assert_eq!(
            *t.get_remembered_value::<UseStateHandle<AttrValue>>(),
            AttrValue::from("item2")
        );

        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "true".to_string().into()
        );

        // Click on the first item
        let radio_item1 = radio_items[0].clone().click().await;

        // Now, the first item should be checked
        assert_eq!(
            *t.get_remembered_value::<UseStateHandle<AttrValue>>(),
            AttrValue::from("item1")
        );

        assert_eq!(
            radio_item1.attribute("aria-checked"),
            "true".to_string().into()
        );
        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "false".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_radio_group_default_value() {
        let t = render!({
            html! {
                <RadioGroup default_value={"item2"}>
                    <RadioGroupItem value={"item1"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>

                    <RadioGroupItem value={"item2"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                </RadioGroup>
            }
        })
        .await;

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 2);

        // The second item should be checked by default
        assert_eq!(
            radio_items[0].attribute("aria-checked"),
            "false".to_string().into()
        );
        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "true".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_radio_group_disabled() {
        let t = render!({
            html! {
                <RadioGroup disabled={true}>
                    <RadioGroupItem value={"item1"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>

                    <RadioGroupItem value={"item2"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                </RadioGroup>
            }
        })
        .await;

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 2);

        for radio_item in &radio_items {
            assert_eq!(
                radio_item.attribute("disabled"),
                "disabled".to_string().into()
            );
        }

        // Attempt to click on the first item
        let radio_item1 = radio_items[0].clone().click().await;

        // The first item should not be checked
        assert_eq!(
            radio_item1.attribute("aria-checked"),
            "false".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_radio_group_item_disabled() {
        let t = render!({
            html! {
                <RadioGroup>
                    <RadioGroupItem value={"item1"} disabled={true}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>

                    <RadioGroupItem value={"item2"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                </RadioGroup>
            }
        })
        .await;

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 2);

        assert_eq!(
            radio_items[0].attribute("disabled"),
            "disabled".to_string().into()
        );
        assert_eq!(radio_items[1].attribute("disabled"), None);

        // Attempt to click on the disabled item
        let radio_item1 = radio_items[0].clone().click().await;

        // The disabled item should not be checked
        assert_eq!(
            radio_item1.attribute("aria-checked"),
            "false".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_radio_group_item_render_as() {
        let t = render!({
            let render_as = Callback::from(|props: RadioGroupItemRenderAsProps| {
                let onclick = {
                    let toggle = props.toggle.clone();
                    Callback::from(move |_event: MouseEvent| {
                        toggle.emit(());
                    })
                };

                html! {
                    <AttrReceiver name="radio-group-item">
                        <input
                            ref={props.r#ref.clone()}
                            type="radio"
                            id={props.id.clone()}
                            class={props.class.clone()}
                            name={props.name.clone()}
                            value={props.value.clone()}
                            checked={props.checked}
                            disabled={props.disabled}
                            readonly={props.readonly}
                            onclick={onclick}
                        />
                    </AttrReceiver>
                }
            });

            html! {
                <RadioGroup name={"test-radio"}>
                    <RadioGroupItem value={"item1"} render_as={render_as.clone()}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                    <RadioGroupItem value={"item2"} render_as={render_as.clone()}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                </RadioGroup>
            }
        })
        .await;

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 2);

        // Initially, no item should be checked
        assert_eq!(
            radio_items[0].attribute("aria-checked"),
            "false".to_string().into()
        );
        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "false".to_string().into()
        );

        // Click on the first item
        let radio_item1 = radio_items[0].clone().click().await;

        // The first item should be checked
        assert_eq!(
            radio_item1.attribute("aria-checked"),
            "true".to_string().into()
        );
        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "false".to_string().into()
        );

        // Click on the second item
        let radio_item2 = radio_items[1].clone().click().await;

        // The second item should be checked, first unchecked
        assert_eq!(
            radio_item1.attribute("aria-checked"),
            "false".to_string().into()
        );

        assert_eq!(
            radio_item2.attribute("aria-checked"),
            "true".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_radio_group_keyboard_navigation() {
        let t = render!({
            html! {
                <RadioGroup name="radio">
                    <RadioGroupItem value={"item1"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                    <RadioGroupItem value={"item2"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                    <RadioGroupItem value={"item3"}>
                        <RadioGroupItemIndicator>{"Checked"}</RadioGroupItemIndicator>
                    </RadioGroupItem>
                </RadioGroup>
            }
        })
        .await;

        let radio_group = t.query_by_role("radiogroup");
        assert!(radio_group.exists());

        let radio_items = t.query_all_by_role("radio");
        assert_eq!(radio_items.len(), 3);

        for radio_item in &radio_items {
            assert_eq!(
                radio_item.attribute("aria-checked"),
                "false".to_string().into()
            );
        }

        let radio_group = radio_group.keydown("ArrowRight").await;
        let radio_items = radio_group.query_all_by_role("radio");

        assert_eq!(
            radio_items[1].attribute("aria-checked"),
            "true".to_string().into()
        );

        let radio_group = radio_group.keydown("ArrowRight").await;
        let radio_items = radio_group.query_all_by_role("radio");

        assert_eq!(
            radio_items[2].attribute("aria-checked"),
            "true".to_string().into()
        );

        let radio_group = radio_group.keydown("ArrowRight").await;
        let radio_items = radio_group.query_all_by_role("radio");

        assert_eq!(
            radio_items[0].attribute("aria-checked"),
            "true".to_string().into()
        );

        let radio_group = radio_group.keydown("ArrowDown").await;
        let radio_items = radio_group.query_all_by_role("radio");

        assert_eq!(
            radio_items[2].attribute("aria-checked"),
            "true".to_string().into()
        );

        let radio_group = radio_group.keydown("ArrowUp").await;
        let radio_items = radio_group.query_all_by_role("radio");

        assert_eq!(
            radio_items[0].attribute("aria-checked"),
            "true".to_string().into()
        );
    }
}
