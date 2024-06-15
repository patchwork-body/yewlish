use icons::*;
use implicit_clone::unsync::IString;
use roving_focus::*;
use toggle::*;
use toggle_group::*;
use utils::Orientation;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct WrapperProps {
    pub title: IString,
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
    pub title: IString,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section class="flex flex-col flex-1 gap-y-5 items-center border rounded-md p-5 border-neutral-600 focus-within:border-neutral-100 text-neutral-400 focus-within:text-neutral-100 hover:text-neutral-100 transition-colors">
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
        data-[state=on]:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent disabled:hover:text-neutral-100
        outline-none transition-colors duration-300
    "##;

    let toggle_group_class = r##"
        flex gap-5 data-[orientation=vertical]:flex-col
    "##;

    let on_pressed_change = Callback::from(|next_state: bool| {
        log::info!("Pressed changed: {:?}", next_state);
    });

    let state = use_state(|| false);

    html! {
        <div class="flex flex-col min-h-screen bg-neutral-950">
            <Wrapper title="Toggle">
                <Section title="Controllable">
                    <Toggle class={toggle_class} pressed={*state} on_pressed_change={Callback::from(move |new_state| {
                        state.set(new_state);
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
        </div>
    }
}
