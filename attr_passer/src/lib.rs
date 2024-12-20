use yew::{prelude::*, virtual_dom::VNode};
use yewlish_synchi::*;

type Attributes = Vec<(&'static str, AttrValue)>;

#[derive(Debug, Clone, PartialEq, Default)]
struct MergeAttributes(Attributes);

impl Merge for MergeAttributes {
    fn merge(&self, other: &Self) -> Self {
        MergeAttributes(self.0.iter().chain(other.0.iter()).cloned().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttrPasserContext {
    pub name: &'static str,
    pub index: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrPasserProps {
    pub name: &'static str,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub attributes: Attributes,
}

#[function_component(AttrPasser)]
pub fn attr_passer(props: &AttrPasserProps) -> Html {
    let channel = use_synchi_channel_with::<MergeAttributes>(
        props.name,
        MergeAttributes(props.attributes.clone()),
    );

    use_effect_with(
        (channel.clone(), props.attributes.clone()),
        |(channel, attributes)| {
            channel
                .borrow_mut()
                .push(MergeAttributes(attributes.clone()));
        },
    );

    let parent_context = use_context::<AttrPasserContext>();

    html! {
        <ContextProvider<AttrPasserContext> context={AttrPasserContext {
            name: channel.borrow().name,
            index: parent_context
                .filter(|ctx| ctx.name == channel.borrow().name)
                .map_or_else(
                    || vec![channel.borrow().index],
                    |ctx| ctx.index.into_iter().chain(std::iter::once(channel.borrow().index)).collect()
                )
        }}>
            {props.children.clone()}
        </ContextProvider<AttrPasserContext>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrReceiverProps {
    #[prop_or_default]
    pub name: &'static str,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AttrReceiver)]
pub fn attr_receiver(props: &AttrReceiverProps) -> Html {
    let context = use_context::<AttrPasserContext>();

    if context.is_none() {
        return html! {
            <>{props.children.clone()}</>
        };
    }

    let context = context.unwrap();

    if props.name != context.name {
        return html! {
            <>{props.children.clone()}</>
        };
    }

    let attributes = use_synchi_channel_subscribe::<MergeAttributes>(
        props.name,
        if context.name == props.name {
            context.index.clone()
        } else {
            vec![]
        },
    );

    if props.children.is_empty() {
        return html! {};
    }

    if props.children.len() > 1 {
        log::warn!("AttrReceiver component only accepts one child");
        return html! {};
    }

    let element = props.children.iter().next().unwrap();

    if let VNode::VTag(tag) = element.clone() {
        let mut tag = (*tag).clone();

        for (key, value) in (*attributes).clone().0 {
            tag.add_attribute(key, value);
        }

        let element = VNode::VTag(Box::new(tag));

        return html! {
            <>{element}</>
        };
    }

    log::warn!("AttrReceiver component only accepts a tag element");

    html! {}
}

#[macro_export]
macro_rules! attributify {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        use yew::props;
        use $crate::AttrPasserProps;

        let mut attributes = vec![];

        $(
            attributes.push(($key, $value.into()));
        )*

        props! {
            AttrPasserProps {
                name: "",
                attributes,
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yewlish_testing_tools::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_attr_passer_for_one_receiver() {
        let t = render!({
            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <AttrReceiver name="test">
                        <div></div>
                    </AttrReceiver>
                </AttrPasser>
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_several_attr_passer_for_one_receiver() {
        let t = render!({
            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <AttrPasser name="test" ..attributify!{ "aria-label" => "button" }>
                        <AttrReceiver name="test">
                            <div></div>
                        </AttrReceiver>
                    </AttrPasser>
                </AttrPasser>
            }
        })
        .await;

        let element = t.query_by_role("button");

        assert!(element.exists());
        assert_eq!(element.attribute("aria-label"), "button".to_string().into());
    }

    #[wasm_bindgen_test]
    async fn test_attr_passer_for_several_receivers() {
        let t = render!({
            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <AttrReceiver name="test">
                        <div></div>
                    </AttrReceiver>

                    <AttrReceiver name="test">
                        <div></div>
                    </AttrReceiver>
                </AttrPasser>
            }
        })
        .await;

        assert_eq!(t.query_all_by_role("button").len(), 1);
    }

    #[wasm_bindgen_test]
    async fn test_nested_attr_passer_with_same_name() {
        let t = render!({
            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <AttrReceiver name="test">
                        <div>
                            <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                                <AttrPasser name="test" ..attributify!{ "aria-label" => "button" }>
                                    <AttrReceiver name="test">
                                        <div></div>
                                    </AttrReceiver>
                                </AttrPasser>
                            </AttrPasser>
                        </div>
                    </AttrReceiver>
                </AttrPasser>
            }
        })
        .await;

        assert_eq!(t.query_all_by_role("button").len(), 2);
        assert_eq!(
            t.query_all_by_role("button")[1].attribute("aria-label"),
            "button".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_neighbor_attr_passer_with_same_name() {
        let t = render!({
            html! {
                <>
                    <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                        <AttrReceiver name="test">
                            <div></div>
                        </AttrReceiver>
                    </AttrPasser>

                    <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                        <AttrReceiver name="test">
                            <div></div>
                        </AttrReceiver>
                    </AttrPasser>
                </>
            }
        })
        .await;

        assert_eq!(t.query_all_by_role("button").len(), 2);
    }

    #[wasm_bindgen_test]
    async fn test_attr_passer_with_mutable_attributes() {
        let t = render!({
            let role = use_state(|| "button".to_string());

            let update_role = use_callback(role.clone(), |_event: MouseEvent, role| {
                role.set("checkbox".to_string());
            });

            html! {
                <AttrPasser name="test" ..attributify! { "role" => (*role).clone() }>
                    <AttrReceiver name="test">
                        <div onclick={&update_role}></div>
                    </AttrReceiver>
                </AttrPasser>
            }
        })
        .await;

        let element = t.query_by_role("button");
        assert!(element.exists());

        element.click().await;

        let element = t.query_by_role("checkbox");
        assert!(element.exists());
    }

    #[wasm_bindgen_test]
    async fn test_attr_passer_with_receiver_in_different_component() {
        #[function_component(AttrReceiverInDifferentComponent)]
        fn attr_receiver_in_different_component() -> Html {
            html! {
                <AttrReceiver name="test">
                    <div></div>
                </AttrReceiver>
            }
        }

        let t = render!({
            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <AttrReceiverInDifferentComponent />
                </AttrPasser>
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_attr_passer_with_receiver_in_different_component_rendered_conditionally() {
        #[derive(Debug, Clone, PartialEq, Properties)]
        struct AttrReceiverInDifferentComponentProps {
            pub show: bool,
        }

        #[function_component(AttrReceiverInDifferentComponent)]
        fn attr_receiver_in_different_component(
            props: &AttrReceiverInDifferentComponentProps,
        ) -> Html {
            if !props.show {
                return html! {};
            }

            html! {
                <AttrReceiver name="test">
                    <div></div>
                </AttrReceiver>
            }
        }

        let t = render!({
            let show = use_state(|| false);

            let toggle_show = use_callback(show.clone(), |_event: MouseEvent, show| {
                show.set(true);
            });

            html! {
                <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                    <div data-testid="trigger" onclick={&toggle_show}>
                        <AttrReceiverInDifferentComponent show={*show} />
                    </div>
                </AttrPasser>
            }
        })
        .await;

        let button = t.query_by_role("button");
        assert!(!button.exists());

        let trigger = t.query_by_testid("trigger");
        trigger.click().await;

        let button = t.query_by_role("button");
        assert!(button.exists());
    }

    #[wasm_bindgen_test]
    async fn test_attr_passer_with_elements_that_mount_and_unmount() {
        let t = render!({
            let show = use_state(|| true);

            let toggle_show = {
                let show = show.clone();
                use_callback((), move |_: MouseEvent, ()| {
                    show.set(!*show);
                })
            };

            html! {
                <>
                    <button onclick={toggle_show.clone()}>{"Toggle"}</button>
                    <AttrPasser name="test" ..attributify!{ "role" => "button" }>
                        { if *show {
                            html! {
                                <AttrReceiver name="test">
                                    <div></div>
                                </AttrReceiver>
                            }
                        } else {
                            html! {}
                        }}
                    </AttrPasser>
                </>
            }
        })
        .await;

        // Initially, the element should exist
        let element = t.query_by_role("button");
        assert!(element.exists());

        // Click the toggle button to unmount the element
        let toggle_button = t.query_by_text("Toggle");
        let toggle_button = toggle_button.click().await;

        // The element should no longer exist
        let element = t.query_by_role("button");
        assert!(!element.exists());

        // Click the toggle button again to remount the element
        toggle_button.click().await;

        // The element should exist again
        let element = t.query_by_role("button");
        assert!(element.exists());
    }
}
