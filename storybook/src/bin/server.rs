#[macro_use]
extern crate rocket;

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use rand::Rng;
use rocket::response::stream::TextStream;
use rocket::tokio::time::{sleep, Duration};
use rocket::{response::content::RawHtml, State};
use rocket_ws::Message;

use storybook::{App, AppProps};

struct IndexHtml {
    pub head: String,
    pub body: String,
}

#[get("/<path..>?<query..>", rank = 11)]
async fn index(
    index_html: &State<IndexHtml>,
    path: PathBuf,
    query: Option<HashMap<String, String>>,
) -> RawHtml<TextStream![String]> {
    let renderer = yew::ServerRenderer::<App>::with_props(move || AppProps {
        path: path.to_string_lossy().to_string().into(),
        query: query.unwrap_or_default().into(),
    });

    let index_html_head = index_html.head.clone();
    let index_html_body = index_html.body.clone();

    RawHtml(TextStream! {
        yield index_html_head;
        yield renderer.render().await;
        yield index_html_body;
    })
}

#[get("/ws/chart")]
async fn chart(ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    rocket_ws::Stream! { ws =>
        loop {
            let random_number: f64 = rand::thread_rng().gen::<f64>() * 100.0;
            yield Message::Text(random_number.to_string());
            sleep(Duration::from_secs(1)).await;
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "dist")]
    dir: PathBuf,
}

#[launch]
async fn rocket() -> _ {
    let args = Args::parse();

    let index_html = tokio::fs::read_to_string(args.dir.join("index.html"))
        .await
        .expect("Failed to read index.html");

    let (index_html_head, index_html_after) = index_html.split_once("<body>").unwrap();
    let mut index_html_head = index_html_head.to_string();
    index_html_head.push_str("<body>");

    rocket::build()
        .manage(IndexHtml {
            head: index_html_head,
            body: index_html_after.to_string(),
        })
        .mount("/", rocket::fs::FileServer::from(args.dir))
        .mount("/", routes![chart, index])
}
