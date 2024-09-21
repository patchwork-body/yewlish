use super::common::*;
use yew::prelude::*;
use yewlish_switch::*;

#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
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

    let switch_state = use_state(|| false);

    html! {
        <Wrapper title="Switch">
            <Section title="Default">
                <Switch class={switch_class}>
                    <SwitchThumb class={switch_thumb_class} />
                </Switch>
            </Section>

            <Section title="Controllable">
                <Switch class={switch_class} checked={*switch_state} onclick={{
                    let switch_state = switch_state.clone();

                    Callback::from(move |_| {
                        switch_state.set(!*switch_state);
                    })
                }}>
                    <SwitchThumb class={switch_thumb_class} />
                </Switch>

                <button onclick={Callback::from(move |_| {
                    switch_state.set(!*switch_state);
                })}>
                    {"Toggle"}
                </button>
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
    }
}
