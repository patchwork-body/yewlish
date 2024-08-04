use std::{collections::HashMap, rc::Rc};

#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

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
                        </ul>
                    </nav>
                </aside>

                <StorybookPage />
            </div>
        </ContextProvider<Router>>
    }
}
