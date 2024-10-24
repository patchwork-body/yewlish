use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewlish_fetch::FetchSchema;
use yewlish_fetch_utils::WsStatus;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
enum Message {
    #[serde(rename = "echo")]
    Echo(String),
    #[serde(rename = "hello")]
    Hello { name: String },
}

impl Default for Message {
    fn default() -> Self {
        Self::Echo(String::new())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
enum Auth {
    #[serde(rename = "login")]
    Login { username: String, password: String },
    #[serde(rename = "logout")]
    Logout,
}

impl Default for Auth {
    fn default() -> Self {
        Self::Login {
            username: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Serialize, Default, Deserialize, PartialEq, Clone, Debug)]
struct QueryStuff {
    query: String,
}

#[derive(FetchSchema)]
enum Api {
    #[ws("/ws", body = Message, res = Message)]
    Messages,
    #[ws("/ws", query = QueryStuff, body = Auth, res = Auth)]
    Auth,
}

#[function_component(Messages)]
pub fn messages() -> Html {
    let events = use_messages(MessagesOpenParams::default());

    let output = match events
        .data
        .as_ref()
        .map_or(Message::default(), std::clone::Clone::clone)
    {
        Message::Echo(body) => html! {
            <p>{format!("Echo: {}", body)}</p>
        },
        Message::Hello { name } => html! {
            <p>{format!("Hello, {}!", name)}</p>
        },
    };

    let input_ref = use_node_ref();

    let onclick = use_callback(
        (events.send.clone(), input_ref.clone()),
        |_: MouseEvent, (send, input_ref)| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();

            send.emit(MessagesParams {
                body: Message::Hello {
                    name: input.value(),
                },
            });
        },
    );

    html! {
        <div>
            <input ref={input_ref} type="text" />
            <button {onclick}>{"Send"}</button>
            <p>
                {output}
            </p>
        </div>
    }
}

#[function_component(AuthCmp)]
fn auth_cmp() -> Html {
    let auth = use_auth(AuthOpenParams::default());

    let opening = *auth.status == WsStatus::Opening;

    let output = match auth
        .data
        .as_ref()
        .map_or(Auth::default(), std::clone::Clone::clone)
    {
        Auth::Login { username, password } => html! {
            <p>{format!("Login: {} {}", username, password)}</p>
        },
        Auth::Logout => html! {
            <p>{"Logout"}</p>
        },
    };

    let login = use_callback(auth.send.clone(), |_: MouseEvent, send| {
        send.emit(AuthParams {
            body: Auth::Login {
                username: "admin".to_string(),
                password: "admin".to_string(),
            },
        });
    });

    html! {
        <div>
            <button onclick={login}>
                {"Login"}
            </button>

            {
                auth.error.as_ref().map(|err| html! {
                    <p>{format!("Error: {}", err)}</p>
                })
            }

            {if opening {
                html! {
                    <p>{"Opening connection..."}</p>
                }
            } else {
                output
            }}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let client = ApiFetchClient::new("http://127.0.0.1:3030");

    html! {
        <ApiFetchClientProvider client={client}>
            <h1>{"Hello, Yew!"}</h1>
            <Messages />
            <Messages />
            <AuthCmp />
        </ApiFetchClientProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
