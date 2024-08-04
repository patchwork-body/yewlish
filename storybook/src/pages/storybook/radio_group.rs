use super::common::*;
use radio_group::*;
use yew::prelude::*;

#[function_component(RadioGroupPage)]
pub fn radio_group_page() -> Html {
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

    html! {
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


    }
}
