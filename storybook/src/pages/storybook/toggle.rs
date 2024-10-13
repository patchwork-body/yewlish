use super::common::*;
use icons::*;
use toggle::*;
use yew::prelude::*;

#[function_component(TogglePage)]
pub fn toggle_page() -> Html {
    let toggle_class = r"
        text-neutral-100 bg-transparent p-1 rounded-md
        hover:text-neutral-400 hover:bg-neutral-800 focus-visible:ring-2 focus-visible:ring-neutral-400
        data-[state=on]:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent
        disabled:hover:text-neutral-100 outline-none transition-colors duration-300
    ";

    let state = use_state(|| false);

    let on_pressed_change = Callback::from(|next_state: bool| {
        log::info!("Pressed changed: {:?}", next_state);
    });

    html! {
        <Wrapper title="Toggle">
            <Section title="Default">
                <Toggle class={toggle_class} on_pressed_change={&on_pressed_change}>
                    <FontItalicIcon width="48" height="48" />
                </Toggle>
            </Section>

            <Section title="Controllable">
                <Toggle class={toggle_class} pressed={*state} on_pressed_change={{
                    let toggle_state = state.clone();

                    Callback::from(move |new_state| {
                        toggle_state.set(new_state);
                    })
                }}>
                    <FontItalicIcon width="48" height="48" />
                </Toggle>

                <button onclick={Callback::from(move |_| {
                    state.set(!*state);
                })}>
                    {"Toggle"}
                </button>
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
    }
}
