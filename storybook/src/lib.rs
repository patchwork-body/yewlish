use checkbox::*;
use icons::*;
use popover::*;
use radio_group::*;
use separator::*;
use switch::*;
use toggle::*;
use toggle_group::*;
use utils::enums::orientation::Orientation;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct WrapperProps {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component(Wrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {
    html! {
        <div class="min-w-screen flex flex-col gap-y-10 p-20">
            <h2 class="text-xl whitespace-nowrap text-neutral-200">{props.title.clone()}</h2>

            <div class="flex flex-wrap items-center gap-10">
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SectionProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let class = classes!(
        "flex",
        "flex-col",
        "flex-1",
        "gap-y-5",
        "items-center",
        "border",
        "rounded-md",
        "p-5",
        "border-neutral-600",
        "focus-within:border-neutral-100",
        "text-neutral-400",
        "focus-within:text-neutral-100",
        "hover:text-neutral-100",
        "transition-colors",
        props.class.as_ref()
    );

    html! {
        <section class={class}>
            <h3 class="text-lg whitespace-nowrap">{props.title.clone()}</h3>
            {props.children.clone()}
        </section>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let toggle_class = r##"
        text-neutral-100 bg-transparent p-1 rounded-md
        hover:text-neutral-400 hover:bg-neutral-800 focus-visible:ring-2 focus-visible:ring-neutral-400
        data-[state=on]:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent
        disabled:hover:text-neutral-100 outline-none transition-colors duration-300
    "##;

    let toggle_group_class = r##"
        flex gap-5 data-[orientation=vertical]:flex-col
    "##;

    let separator_class = r##"
        border-t border-neutral-600 mx-10
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

    let radio_group_class = r##"
        flex flex-row gap-5
    "##;

    let radio_group_inner_class = r##"
        flex flex-row items-center gap-x-2
    "##;

    let radio_group_item_class = r##"
        aspect-square h-4 w-4 rounded-full border border-neutral-100 text-neutral-100
        ring-offset-neutral-950 focus:outline-none focus-visible:ring-2 focus-visible:ring-neutral-100
        focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50
    "##;

    let radio_group_item_indicator_class = r##"
        flex items-center justify-center w-full h-full relative after:content-[''] after:block after:w-[8px]
        after:h-[8px] after:rounded-[50%] after:bg-neutral-100
    "##;

    let on_pressed_change = Callback::from(|next_state: bool| {
        log::info!("Pressed changed: {:?}", next_state);
    });

    let toggle_state = use_state(|| false);
    let checkbox_state = use_state(|| CheckedState::Unchecked);

    html! {
        <div class="flex flex-col min-h-screen bg-neutral-950">
            <Wrapper title="Toggle">
                <Section title="Controllable">
                    <Toggle class={toggle_class} pressed={*toggle_state} on_pressed_change={Callback::from(move |new_state| {
                        toggle_state.set(new_state);
                    })}>
                        <FontItalicIcon width="48" height="48" />
                    </Toggle>
                </Section>

                <Section title="Default">
                    <Toggle class={toggle_class} on_pressed_change={&on_pressed_change}>
                        <FontItalicIcon width="48" height="48" />
                    </Toggle>
                </Section>

                <Section title="Default value">
                    <Toggle class={toggle_class} default_pressed={true} on_pressed_change={&on_pressed_change}>
                        <FontItalicIcon width="48" height="48" />
                    </Toggle>
                </Section>

                <Section title="Disabled">
                    <Toggle class={toggle_class} disabled={true} on_pressed_change={&on_pressed_change}>
                        <FontItalicIcon width="48" height="48" />
                    </Toggle>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="ToggleGroup - Horizontal">
                <Section title="Default value">
                    <ToggleGroup class={toggle_group_class} default_value={vec!["2".into()]}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Checkbox mode">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Not looped">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} r#loop={false}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="No roving focus">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} roving_focus={false}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Disabled">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} disabled={true}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="ToggleGroup - Vertical">
                <Section title="Default value">
                    <ToggleGroup class={toggle_group_class} default_value={vec!["2".into()]} orientation={Orientation::Vertical}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Checkbox mode">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} orientation={Orientation::Vertical}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Not looped">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} r#loop={false} orientation={Orientation::Vertical}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="No roving focus">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} roving_focus={false} orientation={Orientation::Vertical}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>

                <Section title="Disabled">
                    <ToggleGroup class={toggle_group_class} r#type={ToggleGroupType::Checkbox} disabled={true} orientation={Orientation::Vertical}>
                        <ToggleGroupItem class={toggle_class} value="1">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="2">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>

                        <ToggleGroupItem class={toggle_class} value="3">
                            <FontItalicIcon width="48" height="48" />
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="Switch">
                <Section title="Default">
                    <Switch class={switch_class}>
                        <SwitchThumb class={switch_thumb_class} />
                    </Switch>
                </Section>

                <Section title="Default Checked">
                    <Switch class={switch_class} default_checked={true}>
                        <SwitchThumb class={switch_thumb_class} />
                    </Switch>
                </Section>

                <Section title="Disabled">
                    <Switch class={switch_class} disabled={true}>
                        <SwitchThumb class={switch_thumb_class} />
                    </Switch>
                </Section>

                <Section title="Disabled * Checked">
                    <Switch class={switch_class} disabled={true} default_checked={true}>
                        <SwitchThumb class={switch_thumb_class} />
                    </Switch>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="Checkbox">
                <Section title="Default">
                    <div class="flex flex-row items-center gap-x-2">
                        <Checkbox id="checkbox#1" class={checkbox_class}>
                            <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                <CheckIcon />
                            </CheckboxIndicator>
                        </Checkbox>

                        <label for="checkbox#1" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                    </div>
                </Section>

                <Section title="Default value">
                    <div class="flex flex-row items-center gap-x-2">
                        <Checkbox id="checkbox#2" class={checkbox_class} default_checked={CheckedState::Checked}>
                            <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                <CheckIcon />
                            </CheckboxIndicator>
                        </Checkbox>

                        <label for="checkbox#2" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                    </div>
                </Section>

                <Section title="Controlled">
                    <div class="flex flex-row items-center gap-x-2">
                        <Checkbox id="checkbox#3" class={checkbox_class} checked={(*checkbox_state).clone()} on_checked_change={{
                            let checkbox_state = checkbox_state.clone();
                            Callback::from(move |checked: CheckedState| checkbox_state.set(checked))
                        }}>
                            <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                <CheckIcon />
                            </CheckboxIndicator>
                        </Checkbox>

                        <label for="checkbox#3" class={checkbox_label_class}>{"Accept terms and conditions: "} {if *checkbox_state == CheckedState::Checked {"+"} else {"-"}}</label>
                    </div>
                </Section>

                <Section title="Disabled">
                    <div class="flex flex-row items-center gap-x-2">
                        <Checkbox id="checkbox#4" class={checkbox_class} disabled={true}>
                            <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                                <CheckIcon />
                            </CheckboxIndicator>
                        </Checkbox>

                        <label for="checkbox#4" class={checkbox_label_class}>{"Accept terms and conditions: "}</label>
                    </div>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="RadioGroup">
                <Section title="Default">
                    <RadioGroup class={radio_group_class}>
                        <div class={radio_group_inner_class}>
                            <RadioGroupItem id="radio#1" name="radio" value="1" class={radio_group_item_class}>
                                <RadioGroupItemIndicator class={radio_group_item_indicator_class} />
                            </RadioGroupItem>

                            <label for="radio#1" class="text-neutral-200">{"Option 1"}</label>
                        </div>

                        <div class={radio_group_inner_class}>
                            <RadioGroupItem id="radio#2" name="radio" value="2" class={radio_group_item_class}>
                                <RadioGroupItemIndicator class={radio_group_item_indicator_class} />
                            </RadioGroupItem>

                            <label for="radio#2" class="text-neutral-200">{"Option 2"}</label>
                        </div>

                        <div class={radio_group_inner_class}>
                            <RadioGroupItem id="radio#3" name="radio" value="3" class={radio_group_item_class}>
                                <RadioGroupItemIndicator class={radio_group_item_indicator_class} />
                            </RadioGroupItem>

                            <label for="radio#3" class="text-neutral-200">{"Option 3"}</label>
                        </div>
                    </RadioGroup>
                </Section>
            </Wrapper>

            <Separator class={separator_class} />

            <Wrapper title="Popover">
                <Section title="Default">
                    <Popover>
                        <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |params| {
                            let PopoverTriggerRenderAsProps { class, children, toggle, data_state } = params;

                            html! {
                                 <label class={&class} data-state={data_state}>
                                    <Switch class={switch_class} onclick={toggle}>
                                        <SwitchThumb class={switch_thumb_class} />
                                    </Switch>

                                    {children}
                                 </label>
                            }
                        })}>
                            {"Open"}
                        </PopoverTrigger>

                        <PopoverContent class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
                            <div class="p-5 bg-neutral-800 rounded-md">
                                <p class="text-neutral-200">{"Hello, World!"}</p>
                            </div>
                        </PopoverContent>
                    </Popover>
                </Section>

                <Section title="With container">
                    <Popover>
                        <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |params| {
                            let PopoverTriggerRenderAsProps { class, children, toggle, data_state } = params;

                            html! {
                                 <label class={&class} data-state={data_state}>
                                    <Switch class={switch_class} onclick={toggle}>
                                        <SwitchThumb class={switch_thumb_class} />
                                    </Switch>

                                    {children}
                                 </label>
                            }
                        })}>
                            {"Open"}
                        </PopoverTrigger>

                        <PopoverContent container={
                            web_sys::window()
                                .unwrap()
                                .document()
                                .unwrap()
                                .body()
                                .map(|body| body.into())
                        } class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
                            <div class="p-5 bg-neutral-800 rounded-md">
                                <p class="text-neutral-200">{"Hello, World!"}</p>
                            </div>
                        </PopoverContent>
                    </Popover>
                </Section>

                <Section title="With render_as">
                    <Popover>
                        <PopoverTrigger class="flex justify-center gap-x-2" render_as={Callback::from(move |params| {
                            let PopoverTriggerRenderAsProps { class, children, toggle, data_state } = params;

                            html! {
                                 <label class={&class} data-state={data_state}>
                                    <Switch class={switch_class} onclick={toggle}>
                                        <SwitchThumb class={switch_thumb_class} />
                                    </Switch>

                                    {children}
                                 </label>
                            }
                        })}>
                            {"Open"}
                        </PopoverTrigger>

                        <PopoverContent render_as={Callback::from(move |params| {
                            let PopoverContentRenderAsProps { children, class, is_open } = params;

                            if is_open {
                                return html! {
                                    <div class={class}>
                                        {children}
                                    </div>
                                };
                            }

                            html! {
                                { "Popover is closed" }
                            }
                        })} class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out">
                            <div class="p-5 bg-neutral-800 rounded-md">
                                <p class="text-neutral-200">{"Hello, World!"}</p>
                            </div>
                        </PopoverContent>
                    </Popover>
                </Section>
            </Wrapper>
        </div>
    }
}
