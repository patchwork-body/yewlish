use std::{collections::HashMap, rc::Rc};
use yewlish_fetch::FetchSchema;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Default, Serialize, PartialEq, Clone)]
struct PostSlugs {
    id: u32,
}

#[derive(Default, Serialize, PartialEq, Clone)]
struct GetPostCommentsQuery {
    id: u32,
}

#[derive(Default, Deserialize, Debug, Serialize, PartialEq, Clone)]
struct PostBody {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
struct CommentBody {
    id: u32,
    name: String,
    email: String,
    body: String,
    #[serde(rename = "postId")]
    post_id: u32,
}

#[derive(FetchSchema)]
pub enum Api {
    #[get("/posts", res = Vec<PostBody>)]
    GetPosts,
    #[get("/posts/{id}", slugs = PostSlugs, res = PostBody)]
    GetPost,
    #[get("/posts/{id}/comments", slugs = PostSlugs, res = Vec<CommentBody>)]
    GetPostComments,
    #[get("/comments", query = GetPostCommentsQuery, res = Vec<CommentBody>)]
    GetComments,
    #[post("/posts", body = PostBody)]
    CreatePost,
    #[put("/posts/{id}", slugs = PostSlugs, body = PostBody)]
    UpdatePost,
    #[patch("/posts/{id}", slugs = PostSlugs, body = PostBody)]
    PatchPost,
    #[delete("/posts/{id}", slugs = PostSlugs)]
    DeletePost,
    #[ws("/ws")]
    WebSocket,
}

mod pages;

use pages::StorybookPage;

#[derive(Clone, Debug, PartialEq)]
struct Router {
    pub path: AttrValue,
    pub query: Rc<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct AppProps {
    pub path: AttrValue,
    pub query: Rc<HashMap<String, String>>,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let router = use_memo(
        (props.path.clone(), props.query.clone()),
        |(path, query)| {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Router {
                    path: path.into(),
                    query: query.clone(),
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let window = web_sys::window().unwrap_throw();
                let location = window.location();
                let pathname = location.pathname().unwrap_throw();
                let search = location.search().unwrap_throw();

                Router {
                    path: pathname.into(),
                    query: Rc::new(
                        search
                            .trim_start_matches('?')
                            .split('&')
                            .filter_map(|pair| {
                                let mut pair = pair.split('=');

                                let key = pair.next()?;
                                let value = pair.next()?;

                                Some((key.to_string(), value.to_string()))
                            })
                            .collect(),
                    ),
                }
            }
        },
    );

    html! {
        <ContextProvider<Router> context={(*router).clone()}>
            <div class="flex flex-col min-h-screen bg-neutral-950 text-white">
                <aside>
                    <nav>
                        <ul>
                            <li>
                                <a href="/toggle">{"Toggle"}</a>
                            </li>
                            <li>
                                <a href="/checkbox">{"Checkbox"}</a>
                            </li>
                            <li>
                                <a href="/switch">{"Switch"}</a>
                            </li>
                            <li>
                                <a href="/toggle-group">{"Toggle Group"}</a>
                            </li>
                            <li>
                                <a href="/radio-group">{"Radio Group"}</a>
                            </li>
                            <li>
                                <a href="/popover">{"Popover"}</a>
                            </li>
                            <li>
                                <a href="/virtual-list">{"Virtual List"}</a>
                            </li>
                            <li>
                                <a href="/fetch">{"Fetch"}</a>
                            </li>
                        </ul>
                    </nav>
                </aside>

                <StorybookPage />
            </div>
        </ContextProvider<Router>>
    }
}
