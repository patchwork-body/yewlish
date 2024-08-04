use super::common::*;
use checkbox::*;
use icons::*;
use yew::prelude::*;

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let checkbox_container_class = r##"
        flex flex-row items-center gap-x-2
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

    let checkbox_state = use_state(|| CheckedState::Unchecked);

    html! {
        <Wrapper title="Checkbox">
            <Section title="Default">
                <div class={checkbox_container_class}>
                    <Checkbox id="checkbox#1" class={checkbox_class}>
                        <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                            <CheckIcon />
                        </CheckboxIndicator>
                    </Checkbox>

                    <label for="checkbox#1" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                </div>
            </Section>

            <Section title="Default value">
                <div class={checkbox_container_class}>
                    <Checkbox id="checkbox#2" class={checkbox_class} default_checked={CheckedState::Checked}>
                        <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                            <CheckIcon />
                        </CheckboxIndicator>
                    </Checkbox>

                    <label for="checkbox#2" class={checkbox_label_class}>{"Accept terms and conditions"}</label>
                </div>
            </Section>

            <Section title="Controlled">
                <div class={checkbox_container_class}>
                    <Checkbox id="checkbox#3" class={checkbox_class} checked={(*checkbox_state).clone()} on_checked_change={{
                        let checkbox_state = checkbox_state.clone();
                        Callback::from(move |checked: CheckedState| checkbox_state.set(checked))
                    }}>
                        <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                            <CheckIcon />
                        </CheckboxIndicator>
                    </Checkbox>

                    <label for="checkbox#3" class={checkbox_label_class}>
                        {"Accept terms and conditions: "} {if *checkbox_state == CheckedState::Checked {"+"} else {"-"}}
                    </label>

                    <button onclick={Callback::from(move |_| checkbox_state.set(
                        match *checkbox_state {
                            CheckedState::Checked => CheckedState::Unchecked,
                            CheckedState::Unchecked => CheckedState::Checked,
                        }
                    ))}>
                        { "Toggle" }
                    </button>
                </div>
            </Section>

            <Section title="Disabled">
                <div class={checkbox_container_class}>
                    <Checkbox id="checkbox#4" class={checkbox_class} disabled={true}>
                        <CheckboxIndicator class={checkbox_indicator_class} show_when={CheckedState::Checked}>
                            <CheckIcon />
                        </CheckboxIndicator>
                    </Checkbox>

                    <label for="checkbox#4" class={checkbox_label_class}>{"Accept terms and conditions: "}</label>
                </div>
            </Section>
        </Wrapper>
    }
}
