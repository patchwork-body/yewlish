use std::rc::Rc;

use yew::prelude::*;
use yewlish_attr_passer::{attributify, AttrPasser, AttrReceiver};
use yewlish_utils::{
    helpers::combine_handlers::combine_handlers,
    hooks::{use_conditional_attr, use_controllable_state},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchRenderAsProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub children: ChildrenWithProps<SwitchThumb>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub checked: bool,
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
    #[prop_or_default]
    pub toggle: Callback<()>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub children: ChildrenWithProps<SwitchThumb>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub render_as: Option<Callback<SwitchRenderAsProps, Html>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwitchContext {
    pub(crate) checked: bool,
    pub(crate) disabled: bool,
}

pub enum SwitchAction {
    Toggle,
}

impl Reducible for SwitchContext {
    type Action = SwitchAction;

    fn reduce(self: Rc<SwitchContext>, action: Self::Action) -> Rc<SwitchContext> {
        match action {
            SwitchAction::Toggle => SwitchContext {
                checked: !self.checked,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleSwitchContext = UseReducerHandle<SwitchContext>;

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let (checked, dispatch) = use_controllable_state(
        props.default_checked,
        props.checked,
        props.on_checked_change.clone(),
    );

    let context_value = use_reducer(|| SwitchContext {
        checked: *checked.borrow(),
        disabled: props.disabled,
    });

    use_effect_with(
        (*checked.borrow(), context_value.clone()),
        |(checked, context_value)| {
            if *checked != context_value.checked {
                context_value.dispatch(SwitchAction::Toggle);
            }
        },
    );

    let toggle = use_callback(
        (dispatch.clone(), context_value.clone(), props.readonly),
        move |(), (dispatch, context_value, readonly)| {
            if *readonly {
                return;
            }

            dispatch.emit(Box::new(|prev_state| !prev_state));
            context_value.dispatch(SwitchAction::Toggle);
        },
    );

    let toggle_on_click = use_callback(toggle.clone(), move |_: MouseEvent, toggle| {
        toggle.emit(());
    });

    use_conditional_attr(props.r#ref.clone(), "data-disabled", props.disabled);

    let element = if let Some(render_as) = &props.render_as {
        render_as.emit(SwitchRenderAsProps {
            r#ref: props.r#ref.clone(),
            children: props.children.clone(),
            id: props.id.clone(),
            class: props.class.clone(),
            checked: *checked.borrow(),
            disabled: props.disabled,
            required: props.required,
            name: props.name.clone(),
            value: props.value.clone(),
            readonly: props.readonly,
            toggle: toggle.clone(),
        })
    } else {
        html! {
            <AttrReceiver name="switch">
                <button
                    id={&props.id}
                    class={&props.class}
                    type="button"
                    role="switch"
                    disabled={props.disabled}
                    name={&props.name}
                    value={&props.value}
                    onclick={&combine_handlers(props.onclick.clone(), toggle_on_click.into())}
                >
                    {for props.children.iter()}
                </button>
            </AttrReceiver>
        }
    };

    html! {
        <ContextProvider<ReducibleSwitchContext> context={context_value}>
            <AttrPasser name="switch" ..attributify! {
                "aria-checked" => checked.borrow().to_string(),
                "aria-required" => props.required.then_some("true").unwrap_or_default(),
                "data-state" => if *checked.borrow() { "checked" } else { "unchecked" },
                "data-disabled" => props.disabled.to_string(),
            }>
                {element}
            </AttrPasser>
        </ContextProvider<ReducibleSwitchContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchThumbProps {
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(SwitchThumb)]
pub fn switch_thumb(props: &SwitchThumbProps) -> Html {
    let context =
        use_context::<ReducibleSwitchContext>().expect("SwitchThumb must be a child of Switch");

    let data_state = use_memo(context.checked, |checked| {
        if *checked {
            "checked"
        } else {
            "unchecked"
        }
    });

    html! {
        <div
            class={&props.class}
            data-state={*data_state}
            data-disabled={context.disabled.to_string()}
        ></div>
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
    async fn test_switch_should_toggle() {
        let t = render! {
            <Switch>
                <SwitchThumb />
            </Switch>
        }
        .await;

        // The switch should be unchecked by default
        let switch = t.query_by_role("switch");
        assert!(switch.exists());

        assert_eq!(switch.attribute("aria-checked"), Some("false".into()));

        // Click on the switch
        let switch = switch.click().await;

        // Now the switch should be checked
        assert_eq!(switch.attribute("aria-checked"), Some("true".into()));

        // Click again
        let switch = switch.click().await;

        // The switch should be unchecked again
        assert_eq!(switch.attribute("aria-checked"), Some("false".into()));
    }

    // #[wasm_bindgen_test]
    // async fn test_switch_default_checked() {
    //     let t = render! {
    //         <Switch default_checked={Some(true)}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     assert_eq!(switch.attribute("aria-checked"), Some("true".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_default_unchecked() {
    //     let t = render! {
    //         <Switch default_checked={false}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     assert_eq!(switch.attribute("aria-checked"), Some("false".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_checked_prop() {
    //     let t = render! {
    //         <Switch checked={true}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     assert_eq!(switch.attribute("aria-checked"), Some("true".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_is_disabled() {
    //     let t = render! {
    //         <Switch disabled={true}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     assert_eq!(switch.attribute("disabled"), Some("disabled".into()));
    //     assert_eq!(switch.attribute("data-disabled"), Some("true".into()));

    //     // Try clicking the disabled switch
    //     let switch = switch.click().await;

    //     // The state should not have changed
    //     assert_eq!(switch.attribute("aria-checked"), Some("false".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_accept_id() {
    //     let t = render! {
    //         <Switch id={"switch-id"}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());
    //     assert_eq!(switch.attribute("id"), Some("switch-id".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_accept_class() {
    //     let t = render! {
    //         <Switch class={"switch-class"}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());
    //     assert_eq!(switch.attribute("class"), Some("switch-class".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_is_required() {
    //     let t = render! {
    //         <Switch required={true}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());
    //     assert_eq!(switch.attribute("aria-required"), Some("true".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_have_name() {
    //     let t = render! {
    //         <Switch name={"switch-name"}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());
    //     assert_eq!(switch.attribute("name"), Some("switch-name".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_have_value() {
    //     let t = render! {
    //         <Switch value={"switch-value"}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());
    //     assert_eq!(switch.attribute("value"), Some("switch-value".into()));
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_on_checked_change() {
    //     let (h, t) = render_hook!(
    //         (UseStateHandle<bool>, Callback<bool>),
    //         {
    //             let checked = use_state(|| false);

    //             let on_checked_change = {
    //                 let checked = checked.clone();
    //                 Callback::from(move |new_checked: bool| {
    //                     checked.set(new_checked);
    //                 })
    //             };

    //             (checked, on_checked_change)
    //         },
    //         |(checked, on_checked_change): (UseStateHandle<bool>, Callback<bool>)| {
    //             html! {
    //                 <Switch checked={Some(*checked)} on_checked_change={on_checked_change.clone()}>
    //                     <SwitchThumb />
    //                 </Switch>
    //             }
    //         }
    //     )
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     // Initially unchecked
    //     assert_eq!(switch.attribute("aria-checked"), Some("false".into()));
    //     assert!(!*h.get().0);

    //     // Click the switch
    //     let switch = switch.click().await;

    //     // Should now be checked
    //     assert_eq!(switch.attribute("aria-checked"), Some("true".into()));
    //     assert!(*h.get().0);

    //     // Click again
    //     let switch = switch.click().await;

    //     // Should be unchecked again
    //     assert_eq!(switch.attribute("aria-checked"), Some("false".into()));
    //     assert!(!*h.get().0);
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_render_as_input() {
    //     let (_, t) = render_hook!(
    //         Callback<SwitchRenderAsProps, Html>,
    //         {
    //             Callback::from(|props: SwitchRenderAsProps| {
    //                 let onchange = {
    //                     let toggle = props.toggle.clone();
    //                     Callback::from(move |_| {
    //                         toggle.emit(());
    //                     })
    //                 };

    //                 html! {
    //                     <input
    //                         ref={props.r#ref.clone()}
    //                         id={props.id.clone()}
    //                         class={props.class.clone()}
    //                         type="checkbox"
    //                         checked={props.checked}
    //                         disabled={props.disabled}
    //                         required={props.required}
    //                         name={props.name.clone()}
    //                         value={props.value.clone()}
    //                         onchange={onchange}
    //                     />
    //                 }
    //             })
    //         },
    //         |render_as: Callback<SwitchRenderAsProps, Html>| {
    //             html! {
    //                 <Switch {render_as}>
    //                     <SwitchThumb />
    //                 </Switch>
    //             }
    //         }
    //     )
    //     .await;

    //     let input = t.query_by_role("checkbox");
    //     assert!(input.exists());

    //     // Initially unchecked
    //     assert_eq!(input.attribute("checked"), None);

    //     // Click the input
    //     let input = input.click().await;

    //     // Should now be checked
    //     assert_eq!(input.attribute("checked"), "true".to_string().into());

    //     // Click again
    //     let input = input.click().await;

    //     // Should be unchecked again
    //     assert_eq!(input.attribute("checked"), None);
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_readonly() {
    //     let t = render! {
    //         <Switch readonly={true}>
    //             <SwitchThumb />
    //         </Switch>
    //     }
    //     .await;

    //     let switch = t.query_by_role("switch");
    //     assert!(switch.exists());

    //     // Try clicking the readonly switch
    //     let switch = switch.click().await;

    //     // The state should not have changed
    //     assert_eq!(switch.attribute("aria-checked"), "false".to_string().into());
    // }

    // #[wasm_bindgen_test]
    // async fn test_switch_thumb_data_state() {
    //     let t = render! {
    //         <Switch>
    //             <SwitchThumb class={"thumb-class"} />
    //         </Switch>
    //     }
    //     .await;

    //     let thumb = t.query_by_selector(".thumb-class");
    //     assert!(thumb.exists());

    //     // Initially unchecked
    //     assert_eq!(
    //         thumb.attribute("data-state"),
    //         "unchecked".to_string().into()
    //     );

    //     // Click the switch
    //     t.query_by_role("switch").click().await;

    //     // Thumb should now reflect the checked state
    //     assert_eq!(thumb.attribute("data-state"), "checked".to_string().into());
    // }
}
