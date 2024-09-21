use super::common::*;
use popover::*;
use yewlish_switch::{Switch, SwitchThumb};
use virtual_list::VirtualList;
use yew::prelude::*;
use yewlish_attr_passer::AttrReceiver;

#[function_component(VirtualListPage)]
pub fn virtual_list_page() -> Html {
    let total_items = 10_000;
    let item_height = 30.0;
    let viewport_height = 300.0;

    let render_item = Callback::from(move |index: usize| {
        html! {
            <div style="height: 30px; border-bottom: 1px solid #ccc; padding: 5px;">
                { format!("Item {}", index + 1) }
            </div>
        }
    });

    let switch_class = r##"
        peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full
        border-2 border-transparent transition-colors focus-visible:outline-none
        focus-visible:ring-2 focus-visible:ring-neutral-100 focus-visible:ring-offset-2
        focus-visible:ring-offset-neutral-950 disabled:cursor-not-allowed disabled:opacity-50
        data-[state=checked]:bg-neutral-100 data-[state=unchecked]:bg-neutral-800
    "##;

    let switch_thumb_class = r##"
        pointer-events-none block h-5 w-5 rounded-full bg-neutral-950 shadow-lg ring-0 transition-transform
        data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0
    "##;

    html! {
        <Wrapper title="Toggle">
            <Section title="Default">
                <Popover>
                    <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |PopoverTriggerRenderAsProps { class, children, toggle, is_open }| {
                        html! {
                            <AttrReceiver name="popover-trigger">
                                <label class={&class}>
                                    <Switch class={switch_class} onclick={toggle} checked={is_open}>
                                        <SwitchThumb class={switch_thumb_class} />
                                    </Switch>

                                    {children}
                                </label>
                            </AttrReceiver>
                        }
                    })}>
                        {"Open"}
                    </PopoverTrigger>

                    <PopoverContent
                        class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out bg-neutral-900 mt-2 min-w-md rounded-md p-2"
                    >
                        <VirtualList
                            total_items={total_items}
                            item_height={item_height}
                            viewport_height={viewport_height}
                            render_item={&render_item}
                        />
                    </PopoverContent>
                </Popover>
            </Section>
        </Wrapper>
    }
}
