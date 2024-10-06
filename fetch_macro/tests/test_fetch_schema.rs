use serde::{Deserialize, Serialize};
use wasm_bindgen_test::wasm_bindgen_test;
use yew::prelude::*;
use yewlish_fetch_macro::FetchSchema;
use yewlish_testing_tools::*;

#[derive(Default, Serialize)]
struct GetPostsQueryParams {
    page: u32,
}

#[derive(Default, Serialize)]
struct CreatePostBody {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct Post {
    title: String,
    content: String,
}

#[derive(FetchSchema)]
enum Test {
    #[get("/test", query = GetPostsQueryParams, body = (), res = &'static str)]
    GetPosts,
    #[post("/test2", query = (), body = CreatePostBody, res = &'static str)]
    CreatePost,
}

#[test]
fn test_fetch_schema() {
    let client = TestFetchClient::new("".to_string());

    client.get_posts(GetPostsParams {
        query: GetPostsQueryParams { page: 1 },
        ..Default::default()
    });
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_fetch_schema_hooks() {
    let t = render!({
        let res = use_create_post(CreatePostParams {
            body: CreatePostBody {
                title: "Hello".to_string(),
                content: "World".to_string(),
            },
            ..Default::default()
        });

        use_remember_value(res);

        html! {}
    })
    .await;

    assert_eq!(t.get_state::<&'static str>(), "/test2");
}
