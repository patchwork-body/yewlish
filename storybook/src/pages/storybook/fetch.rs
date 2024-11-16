use yew::prelude::*;

use crate::{
    pages::storybook::common::{Section, Wrapper},
    use_get_posts, ApiFetchClient, ApiFetchClientDebug, ApiFetchClientProvider, GetPostsParams,
};

#[function_component(FetchPage)]
pub fn fetch_page() -> Html {
    let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

    html! {
        <ApiFetchClientProvider client={client}>
            <Wrapper title="Fetch">
                <Section title="Debug">
                    <GetPosts />
                    <ApiFetchClientDebug />
                </Section>
            </Wrapper>
        </ApiFetchClientProvider>
    }
}

#[function_component(GetPosts)]
fn get_posts() -> Html {
    let posts = use_get_posts(GetPostsParams::default());

    html! {
        <div>
            <h2>{ "Posts" }</h2>
            <ul>
                { for (*posts.data).iter().map(|posts| html! {
                    { for posts.iter().map(|post| html! {
                        <li key={post.id}>
                            <h3 style={"text-transform: uppercase; text-decoration: underline;".to_string()}>{ &post.title }</h3>
                            <p>{ &post.body }</p>
                        </li>
                    }) }
                }) }
            </ul>
        </div>
    }
}
