use super::common::*;
use icons::*;
use popover::*;
use switch::*;
use yew::prelude::*;
use yewlish_attr_passer::*;
use yewlish_checkbox::*;

#[function_component(PopoverPage)]
pub fn popover_page() -> Html {
    let checkbox_class = r##"
        peer h-4 w-4 shrink-0 rounded-sm border border-neutral-100 ring-offset-neutral-950 focus-visible:outline-none
        focus-visible:ring-2 focus-visible:ring-neutral-100 focus-visible:ring-offset-2 disabled:cursor-not-allowed
        disabled:opacity-50 data-[state=checked]:bg-neutral-100 data-[state=checked]:text-neutral-950
    "##;

    let checkbox_label_class = r##"
        text-neutral-200 text-nowrap
    "##;

    let checkbox_indicator_class = r##"
        flex items-center justify-center text-current
    "##;

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

    #[cfg(target_arch = "wasm32")]
    let popover_container = use_state(|| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("container-for-popover")
    });

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(target_arch = "wasm32")]
    {
        let popover_container = popover_container.clone();

        use_effect(move || {
            if popover_container.is_none() {
                popover_container.set(
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("container-for-popover"),
                );
            }
        });
    }

    let listbox_option_class = r##"
        flex items-center gap-2 p-2 hover:bg-neutral-700 hover:text-neutral-100 cursor-pointer
        aria-[checked=true]:bg-neutral-850 data-[active="true"]:bg-neutral-800
    "##;

    let listbox_option_indicator_class = r##"
        flex items-center justify-center text-current ring-1 rounded-md ring-white w-5 h-5
    "##;

    let popover_state = use_state(|| true);

    html! {
        <Wrapper title="Popover">
            <Section title="Default">
                <Popover>
                    <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |PopoverTriggerRenderAsProps { class, children, toggle, is_open }| {
                        html! {
                            <AttrReceiver>
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

                    <PopoverContent class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
                        <div class="p-5 bg-neutral-800 rounded-md">
                            <p class="text-neutral-200">{"Hello, World!"}</p>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#1" class={checkbox_class} default_checked={CheckedState::Checked}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#1" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#2" class={checkbox_class}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#2" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#3" class={checkbox_class}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#3" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>
                        </div>
                    </PopoverContent>
                </Popover>
            </Section>

            <Section title="With container">
                <Popover>
                    <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |PopoverTriggerRenderAsProps { class, children, toggle, is_open }| {
                        html! {
                            <AttrReceiver>
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

                    <PopoverContent class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
                        <div class="p-5 bg-neutral-800 rounded-md">
                            <p class="text-neutral-200">{"Hello, World!"}</p>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#4" class={checkbox_class} default_checked={CheckedState::Checked}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#4" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#5" class={checkbox_class}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#5" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>

                            <div class="flex flex-row items-center gap-x-2">
                                <Checkbox id="popover-checkbox#6" class={checkbox_class}>
                                    <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                        <CheckIcon />
                                    </CheckboxIndicator>
                                </Checkbox>

                                <label for="popover-checkbox#6" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                            </div>
                        </div>
                    </PopoverContent>
                </Popover>

                <span id="container-for-popover">
                    { "Container for Popover" }
                </span>
            </Section>

            // <Section title="With render_as">
            //     <Popover>
            //         <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |PopoverTriggerRenderAsProps { class, children, toggle, is_open }| {
            //             html! {
            //                 <AttrReceiver>
            //                     <label class={&class}>
            //                         <Switch class={switch_class} onclick={toggle} checked={is_open}>
            //                             <SwitchThumb class={switch_thumb_class} />
            //                         </Switch>

            //                         {children}
            //                     </label>
            //                 </AttrReceiver>
            //             }
            //         })}>
            //             {"Open"}
            //         </PopoverTrigger>

            //         <PopoverContent on_esc_key_down={Callback::from(move |event: KeyboardEvent| {
            //             event.prevent_default();
            //         })} render_as={Callback::from(move |PopoverContentRenderAsProps { r#ref, children, class, is_open }| {
            //             if is_open {
            //                 return html! {
            //                     <AttrReceiver>
            //                         <div ref={r#ref} class={class}>
            //                             {children}
            //                         </div>
            //                     </AttrReceiver>
            //                 };
            //             }

            //             html! {
            //                 { "Popover is closed" }
            //             }
            //         })} class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
            //             <div class="p-5 bg-neutral-800 rounded-md">
            //                 <p class="text-neutral-200">{"Hello, World!"}</p>

            //                 <div class="flex flex-row items-center gap-x-2">
            //                     <Checkbox id="popover-checkbox#7" class={checkbox_class} default_checked={CheckedState::Checked}>
            //                         <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
            //                             <CheckIcon />
            //                         </CheckboxIndicator>
            //                     </Checkbox>

            //                     <label for="popover-checkbox#7" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
            //                 </div>

            //                 <div class="flex flex-row items-center gap-x-2">
            //                     <Checkbox id="popover-checkbox#8" class={checkbox_class}>
            //                         <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
            //                             <CheckIcon />
            //                         </CheckboxIndicator>
            //                     </Checkbox>

            //                     <label for="popover-checkbox#8" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
            //                 </div>

            //                 <div class="flex flex-row items-center gap-x-2">
            //                     <Checkbox id="popover-checkbox#9" class={checkbox_class}>
            //                         <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
            //                             <CheckIcon />
            //                         </CheckboxIndicator>
            //                     </Checkbox>

            //                     <label for="popover-checkbox#9" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
            //                 </div>
            //             </div>
            //         </PopoverContent>
            //     </Popover>
            // </Section>

            // <Section title="Combobox">
            //     <Popover>
            //         <PopoverTrigger class="flex justify-center gap-x-2 text-white border rounded-md px-2 py-1">
            //             {"Open"}
            //         </PopoverTrigger>

            //         <PopoverContent class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out bg-neutral-900 rounded-md shadow-sm">
            //             <Listbox multiple={true} class="min-w-[200px]" on_selected_change={Callback::from(|selected| {
            //                 log::info!("Selected: {:?}", selected);
            //             })}>
            //                 <ListboxOption id="listbox-option-#1" class={listbox_option_class}>
            //                     <ListboxOptionIndicator class={listbox_option_indicator_class}>
            //                         <CheckIcon />
            //                     </ListboxOptionIndicator>

            //                     {"Option 1"}
            //                 </ListboxOption>

            //                 <ListboxOption id="listbox-option-#2" class={listbox_option_class}>
            //                     <ListboxOptionIndicator class={listbox_option_indicator_class}>
            //                         <CheckIcon />
            //                     </ListboxOptionIndicator>

            //                     {"Option 2"}
            //                 </ListboxOption>

            //                 <ListboxOption id="listbox-option-#3" class={listbox_option_class}>
            //                     <ListboxOptionIndicator class={listbox_option_indicator_class}>
            //                         <CheckIcon />
            //                     </ListboxOptionIndicator>

            //                     {"Option 3"}
            //                 </ListboxOption>
            //             </Listbox>
            //         </PopoverContent>
            //     </Popover>
            // </Section>
        </Wrapper>
    }
}
