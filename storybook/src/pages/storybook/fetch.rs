use chrono::{DateTime, Utc};
use web_sys::CloseEvent;
use yew::prelude::*;
use yewlish_fetch_utils::{use_signal_state, Signal, WsStatus};

use crate::{
    pages::storybook::common::{Section, Wrapper},
    use_chart_with_options, use_chart_with_options_async, use_get_posts, ApiFetchClient,
    ApiFetchClientDebug, ApiFetchClientProvider, ChartOptions, ChartParams, GetPostsParams,
    OnChartUpdate, WsApiFetchClient, WsApiFetchClientProvider,
};

#[function_component(FetchPage)]
pub fn fetch_page() -> Html {
    let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");
    let ws_client = WsApiFetchClient::new("ws://localhost:8000");

    html! {
        <ApiFetchClientProvider client={client}>
            <WsApiFetchClientProvider client={ws_client}>
                <Wrapper title="jsonplaceholder">
                    <Section title="Get Posts">
                        <GetPosts />
                    </Section>
                </Wrapper>

                <Wrapper title="web sockets">
                    <Section title="data">
                        <WebSocket />
                    </Section>
                </Wrapper>

                <ApiFetchClientDebug />
            </WsApiFetchClientProvider>
        </ApiFetchClientProvider>
    }
}

#[function_component(GetPosts)]
fn get_posts() -> Html {
    let posts = use_get_posts(GetPostsParams::default());

    html! {
        <div class="max-h-[300px] overflow-y-auto">
            <h2>{ "Posts" }</h2>
            <ul>
                { for (*posts.data).iter().map(|posts| html! {
                    { for posts.iter().map(|post| html! {
                        <li class="max-h-[200px] overflow-hidden" key={post.id}>
                            <h3 class="uppercase underline">{ &post.title }</h3>
                            <p>{ &post.body }</p>
                        </li>
                    }) }
                }) }
            </ul>
        </div>
    }
}

#[derive(Clone, Debug)]
struct DataPoint {
    value: f64,
    timestamp: DateTime<Utc>,
}

#[function_component(WebSocket)]
fn web_socket() -> Html {
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

    let chart = use_chart_with_options_async(ChartOptions {
        on_update: on_update.into(),
        on_status_change: on_status.into(),
        on_close: on_close.into(),
        ..Default::default()
    });

    let width = 600.0;
    let height = 300.0;
    let padding = 20.0;

    let points = (*data)
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let x = padding + (i as f64 / data.len() as f64) * (width - 2.0 * padding);
            let y = height - (padding + (point.value / 100.0) * (height - 2.0 * padding));
            format!("{},{}", x, y)
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

    log::debug!("Error: {:?}", chart.error);

    html! {
        <div>
            { if *chart.status == WsStatus::Closed {
                html! {
                    <button onclick={&start_ws}>{ "Start" }</button>
                }
            } else if *chart.status == WsStatus::Open {
                html! {
                    <button onclick={&stop_ws}>{ "Stop" }</button>
                }
            } else if *chart.status == WsStatus::Opening {
                html! {
                    <p>{ "Connecting..." }</p>
                }
            } else if *chart.status == WsStatus::Closing {
                html! {
                    <p>{ "Disconnecting..." }</p>
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
