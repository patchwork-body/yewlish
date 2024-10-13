use super::common::*;
use icons::*;
use yew::prelude::*;
use yewlish_toggle_group::*;
use yewlish_utils::enums::Orientation;

#[function_component(ToggleGroupPage)]
pub fn toggle_group_page() -> Html {
    let toggle_class = r"
        text-neutral-100 bg-transparent p-1 rounded-md
        hover:text-neutral-400 hover:bg-neutral-800 focus-visible:ring-2 focus-visible:ring-neutral-400
        data-[state=on]:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent
        disabled:hover:text-neutral-100 outline-none transition-colors duration-300
    ";

    let toggle_group_class = r"
        flex gap-5 data-[orientation=vertical]:flex-col
    ";

    html! {
        <>
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

                <Section title="Controllable">
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
        </>
    }
}
