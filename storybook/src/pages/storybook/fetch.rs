use std::rc::Rc;

use chrono::{DateTime, Utc};
use web_sys::CloseEvent;
use yew::prelude::*;
use yewlish_fetch_utils::{
    use_signal_state, CacheOptions, CachePolicy, FetchError, Signal, WsStatus,
};

use crate::{
    pages::storybook::common::{Section, Wrapper},
    use_api_fetch_client, use_chart_with_options_async, use_create_post_with_options_async,
    use_get_posts, use_get_posts_with_options, use_get_posts_with_options_async, ApiFetchClient,
    ApiFetchClientDebug, ApiFetchClientProvider, ChartOptions, ChartParams, CreatePostOptions,
    CreatePostParams, GetPostsOptions, GetPostsParams, OnChartUpdate, OnCreatePostUpdate, PostBody,
    WsApiFetchClient, WsApiFetchClientProvider,
};

#[function_component(FetchPage)]
pub fn fetch_page() -> Html {
    let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");
    let ws_client = WsApiFetchClient::new("ws://localhost:8000");

    html! {
        <ApiFetchClientProvider client={client}>
            <Wrapper title="jsonplaceholder">
                <Section title="Live">
                    <CreatePost />
                    <GetPosts />
                </Section>

                <Section title="Cached">
                    <CachedGetPosts />
                </Section>
            </Wrapper>

            <AppStateProvider>
                <Wrapper title="With App State">
                    <Section title="Yewlish-fetch">
                        <CreatePostWithAppState />
                        <GetPostsWithAppState />
                    </Section>

                    <Section title="Reducer">
                        <GetPostsFromAppState />
                    </Section>
                </Wrapper>
            </AppStateProvider>

            <WsApiFetchClientProvider client={ws_client}>
                <Wrapper title="Web Sockets">
                    <Section title="charts">
                        <Charts />
                    </Section>
                </Wrapper>
            </WsApiFetchClientProvider>

            <ApiFetchClientDebug />
        </ApiFetchClientProvider>
    }
}

#[function_component(GetPosts)]
fn get_posts() -> Html {
    let posts = use_get_posts(GetPostsParams::default());

    html! {
        <div class="max-h-[300px] overflow-y-auto">
            <ul>
                {(*posts.data).clone().unwrap_or_default().iter().map(|post| {
                    html! {
                        <li class="max-h-[200px] overflow-hidden">
                            <h3 class="uppercase underline">{ &post.title }</h3>
                            <p>{ &post.body }</p>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

#[function_component(CachedGetPosts)]
fn cached_get_posts() -> Html {
    let posts = use_get_posts_with_options_async(GetPostsOptions {
        cache_options: CacheOptions {
            policy: Some(CachePolicy::CacheOnly),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    let refresh = use_callback(posts.trigger.clone(), |event: MouseEvent, trigger| {
        event.prevent_default();
        trigger.emit(GetPostsParams::default());
    });

    html! {
        <div class="max-h-[300px] overflow-y-auto">
            <button onclick={&refresh}>
                { "Refresh" }
            </button>

            <ul>
                {(*posts.data).clone().unwrap_or_default().iter().map(|post| {
                    html! {
                        <li class="max-h-[200px] overflow-hidden">
                            <h3 class="uppercase underline">{ &post.title }</h3>
                            <p>{ &post.body }</p>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

#[function_component(CreatePost)]
fn create_post() -> Html {
    let client = use_api_fetch_client();

    let on_created = use_callback(client.clone(), |update: OnCreatePostUpdate, client| {
        client.update_get_posts_queries(|query| {
            query.map(|data| {
                data.into_iter()
                    .chain(std::iter::once(update.incoming.clone()))
                    .collect()
            })
        });

        Some(update.incoming)
    });

    let create_post = use_create_post_with_options_async(CreatePostOptions {
        on_update: on_created.into(),
        ..Default::default()
    });

    let submit = use_callback(
        create_post.trigger.clone(),
        |event: SubmitEvent, create_post| {
            event.prevent_default();

            let Some(form_element) = event.target_dyn_into::<web_sys::HtmlFormElement>() else {
                return;
            };

            let Ok(form_data) = web_sys::FormData::new_with_form(&form_element) else {
                return;
            };

            let title = form_data.get("title").as_string().unwrap_or_default();
            let body = form_data.get("body").as_string().unwrap_or_default();

            create_post.emit(CreatePostParams {
                body: PostBody {
                    id: 0,
                    title,
                    body,
                    user_id: 0,
                },
                ..Default::default()
            });
        },
    );

    html! {
        <form class="flex flex-col gap-y-4" onsubmit={&submit}>
            <input class="border rounded-md p-2 bg-neutral-950 text-white" name="title" type="text" placeholder="Title" />
            <textarea class="border rounded-md p-2 bg-neutral-950 text-white" name="body" placeholder="Body"></textarea>
            <button class="border rounded-md p-2" type="submit">{ "Submit" }</button>
        </form>
    }
}

#[derive(Clone, Default, PartialEq)]
struct AppState {
    pub posts: Vec<PostBody>,
}

enum AppStateAction {
    InitPosts(Vec<PostBody>),
    AddPost(PostBody),
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::InitPosts(posts) => Rc::new(Self { posts }),
            AppStateAction::AddPost(post) => {
                let mut posts = self.posts.clone();
                posts.push(post);
                Rc::new(Self { posts })
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppStateProviderProps {
    pub children: Children,
}

type ReducibleAppState = UseReducerHandle<AppState>;

#[function_component(AppStateProvider)]
pub fn app_state_provider(props: &AppStateProviderProps) -> Html {
    let app_state = use_reducer(AppState::default);

    html! {
        <ContextProvider<ReducibleAppState> context={app_state}>
            { props.children.clone() }
        </ContextProvider<ReducibleAppState>>
    }
}

#[function_component(GetPostsWithAppState)]
fn get_posts_with_app_state() -> Html {
    let app_state = use_context::<ReducibleAppState>().expect("No app state found");

    let init_app_state = use_callback((), {
        let app_state = app_state.clone();

        move |posts: Vec<PostBody>, ()| {
            app_state.dispatch(AppStateAction::InitPosts(posts));
        }
    });

    let posts = use_get_posts_with_options(
        GetPostsParams::default(),
        GetPostsOptions {
            on_data: init_app_state.into(),
            ..Default::default()
        },
    );

    html! {
        <div class="max-h-[300px] overflow-y-auto">
            <ul>
                {(*posts.data).clone().unwrap_or_default().iter().map(|post| {
                    html! {
                        <li class="max-h-[200px] overflow-hidden">
                            <h3 class="uppercase underline">{ &post.title }</h3>
                            <p>{ &post.body }</p>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

#[function_component(CreatePostWithAppState)]
fn create_post_with_app_state() -> Html {
    let app_state = use_context::<ReducibleAppState>().expect("No app state found");

    let on_created = use_callback((), {
        let app_state = app_state.clone();

        move |update: OnCreatePostUpdate, ()| {
            app_state.dispatch(AppStateAction::AddPost(update.incoming.clone()));
            Some(update.incoming)
        }
    });

    let create_post = use_create_post_with_options_async(CreatePostOptions {
        on_update: on_created.into(),
        ..Default::default()
    });

    let submit = use_callback(
        create_post.trigger.clone(),
        |event: SubmitEvent, create_post| {
            event.prevent_default();

            let Some(form_element) = event.target_dyn_into::<web_sys::HtmlFormElement>() else {
                return;
            };

            let Ok(form_data) = web_sys::FormData::new_with_form(&form_element) else {
                return;
            };

            let name = form_data.get("title").as_string().unwrap_or_default();
            let number = form_data.get("body").as_string().unwrap_or_default();

            create_post.emit(CreatePostParams {
                body: PostBody {
                    id: 0,
                    title: name,
                    body: number,
                    user_id: 0,
                },
                ..Default::default()
            });
        },
    );

    html! {
        <form class="flex flex-col gap-y-4" onsubmit={&submit}>
            <input class="border rounded-md p-2 bg-neutral-950 text-white" name="title" type="text" placeholder="Title" />
            <textarea class="border rounded-md p-2 bg-neutral-950 text-white" name="body" placeholder="Body"></textarea>
            <button class="border rounded-md p-2" type="submit">{ "Submit" }</button>
        </form>
    }
}

#[function_component(GetPostsFromAppState)]
pub fn get_posts_from_app_state() -> Html {
    let app_state = use_context::<ReducibleAppState>().expect("No app state found");

    html! {
        <div class="max-h-[300px] overflow-y-auto">
            <ul>
                {
                    app_state.posts.iter().map(|post| {
                        html! {
                            <li>
                                <h3>{ &post.title }</h3>
                                <p>{ &post.body }</p>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

#[function_component(Charts)]
pub fn charts() -> Html {
    let charts_vec = use_state(std::vec::Vec::new);

    let inc = use_callback(charts_vec.clone(), {
        |event: MouseEvent, charts_vec| {
            event.prevent_default();
            let mut charts_vec_data = (**charts_vec).clone();
            charts_vec_data.push(uuid::Uuid::new_v4());
            charts_vec.set(charts_vec_data);
        }
    });

    html! {
        <div>
            <div class="flex justify-center gap-2">
                <button class="border rounded-lg p-2" onclick={&inc}>{ "Add" }</button>
            </div>

            <ul class="flex flex-wrap justify-center">
                { for charts_vec.iter().map(|id| {
                    html! {
                        <li data-id={id.to_string()} key={id.to_string()}>
                            <Chart />
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}

#[derive(Clone, Debug)]
struct DataPoint {
    pub value: f64,
    #[allow(dead_code)]
    pub timestamp: DateTime<Utc>,
}

#[function_component(Chart)]
fn chart() -> Html {
    let signal = use_mut_ref(|| Signal::<Vec<DataPoint>>::new(vec![]));
    let data = use_signal_state(signal.clone());

    let on_update = use_callback((), {
        let signal = signal.clone();

        move |update: OnChartUpdate, ()| {
            let mut data = signal.borrow().get();

            data.push(DataPoint {
                value: update.incoming,
                timestamp: Utc::now(),
            });

            signal.borrow().set(data);

            Some(update.incoming)
        }
    });

    let on_status = use_callback((), |next_status: WsStatus, ()| {
        log::info!("Status: {:?}", next_status);
    });

    let on_close = use_callback((), |event: CloseEvent, ()| {
        log::info!("Close event: {:?}", event);
    });

    let on_error = use_callback((), |error: FetchError, ()| {
        log::error!("Error: {:?}", error);
    });

    let chart = use_chart_with_options_async(ChartOptions {
        on_update: on_update.into(),
        on_status_change: on_status.into(),
        on_close: on_close.into(),
        on_error: on_error.into(),
        ..Default::default()
    });

    let width = 600.0;
    let height = 300.0;
    let padding = 20.0;

    let points = (*data)
        .iter()
        .enumerate()
        .map(|(i, point)| {
            #[allow(clippy::cast_precision_loss)]
            let x = padding + (i as f64 / data.len() as f64) * (width - 2.0 * padding);
            let y = height - (padding + (point.value / 100.0) * (height - 2.0 * padding));
            format!("{x},{y}")
        })
        .collect::<Vec<_>>();

    let start_ws = use_callback(chart.open.clone(), |event: MouseEvent, open| {
        event.prevent_default();
        open.emit(ChartParams::default());
    });

    let stop_ws = use_callback(chart.close.clone(), |event: MouseEvent, close| {
        event.prevent_default();
        close.emit(());
    });

    html! {
        <div class="border border-lg m-4 p-4">
            { if *chart.status == WsStatus::Closed {
                html! {
                    <button class="border rounded-lg p-2" onclick={&start_ws}>{ "Start" }</button>
                }
            } else if *chart.status == WsStatus::Open {
                html! {
                    <button class="border rounded-lg p-2" onclick={&stop_ws}>{ "Stop" }</button>
                }
            } else {
                html! {}
            } }

            <svg width={width.to_string()} height={height.to_string()}>
                if points.len() > 1 {
                    <polyline
                        points={points.join(" ")}
                        fill="none"
                        stroke="#0074d9"
                        stroke-width="2"
                    />
                }
                { for points.iter().map(|point| {
                    html! {
                        <circle
                            cx={point.split(',').next().unwrap().to_string()}
                            cy={point.split(',').nth(1).unwrap().to_string()}
                            r="3"
                            fill="#0074d9"
                        />
                    }
                }) }
            </svg>
        </div>
    }
}
