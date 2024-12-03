use schema::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;
use yewlish_fetch_utils::*;
use yewlish_testing_tools::*;

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

mod schema {
    use yewlish_fetch::FetchSchema;

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
        #[ws("/ws", res = f64)]
        WebSocket,
        #[ws("/ws", res = String)]
        WebSocket2,
    }
}

pub use schema::Api;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, Clone, PartialEq)]
struct TestRootProps {
    children: Children,
}

#[function_component(TestRoot)]
fn test_root(props: &TestRootProps) -> Html {
    let client =
        ApiFetchClient::new("https://jsonplaceholder.typicode.com").with_middlewares(vec![
            Rc::new(|_request_init, headers| {
                let headers = headers.clone();

                // Of course this is a dummy middleware, but it's just for testing purposes
                let future = async move {
                    headers
                        .borrow_mut()
                        .set("Authorization", "Bearer token")
                        .unwrap();
                };

                Box::pin(future)
            }),
        ]);

    html! {
        <ApiFetchClientProvider client={client}>
            <div class="test-root">
                {for props.children.iter()}
            </div>
        </ApiFetchClientProvider>
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_get_request() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = &client
            .get_posts(
                client.prepare_get_posts_url(),
                signal.clone(),
                GetPostsParams::default(),
            )
            .await;

        assert!(result.is_ok());

        let result: Vec<PostBody> =
            deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    async fn test_get_request_with_slugs() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = &client
            .get_post(
                client.prepare_get_post_url(),
                signal.clone(),
                GetPostParams {
                    slugs: PostSlugs { id: 1 },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());

        let result: PostBody = deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
        assert_eq!(result.id, 1);
    }

    #[wasm_bindgen_test]
    async fn test_get_request_with_query() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = &client
            .get_post_comments(
                client.prepare_get_comments_url(),
                signal.clone(),
                GetPostCommentsParams {
                    slugs: PostSlugs { id: 1 },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());

        let result: Vec<CommentBody> =
            deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
        assert_eq!(result.len(), 500);
    }

    #[wasm_bindgen_test]
    async fn test_post_request() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = client
            .create_post(
                client.prepare_create_post_url(),
                signal.clone(),
                CreatePostParams {
                    body: PostBody {
                        id: 101,
                        title: "Test".to_string(),
                        body: "Test".to_string(),
                        user_id: 1,
                    },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());

        let result: PostBody = deserialize_response(result.unwrap().as_str()).unwrap();
        assert_eq!(result.id, 101);
        assert_eq!(result.title, "Test".to_string());
        assert_eq!(result.body, "Test".to_string());
        assert_eq!(result.user_id, 1);
    }

    #[wasm_bindgen_test]
    async fn test_put_request() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = client
            .update_post(
                client.prepare_update_post_url(),
                signal.clone(),
                UpdatePostParams {
                    slugs: PostSlugs { id: 1 },
                    body: PostBody {
                        id: 1,
                        title: "Test".to_string(),
                        body: "Test".to_string(),
                        user_id: 1,
                    },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());

        let result: PostBody = deserialize_response(result.unwrap().as_str()).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.title, "Test".to_string());
        assert_eq!(result.body, "Test".to_string());
        assert_eq!(result.user_id, 1);
    }

    #[wasm_bindgen_test]
    async fn test_patch_request() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = client
            .patch_post(
                client.prepare_patch_post_url(),
                signal.clone(),
                PatchPostParams {
                    slugs: PostSlugs { id: 1 },
                    body: PostBody {
                        id: 1,
                        title: "Test".to_string(),
                        body: "Test".to_string(),
                        user_id: 1,
                    },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());

        let result: PostBody = deserialize_response(result.unwrap().as_str()).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.title, "Test".to_string());
        assert_eq!(result.body, "Test".to_string());
        assert_eq!(result.user_id, 1);
    }

    #[wasm_bindgen_test]
    async fn test_delete_request() {
        let client = ApiFetchClient::new("https://jsonplaceholder.typicode.com");

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = client
            .delete_post(
                client.prepare_delete_post_url(),
                signal.clone(),
                DeletePostParams {
                    slugs: PostSlugs { id: 1 },
                    ..Default::default()
                },
            )
            .await;

        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_middleware() {
        let client =
            ApiFetchClient::new("https://jsonplaceholder.typicode.com").with_middlewares(vec![
                Rc::new(|_request_init, headers| {
                    let headers = headers.clone();

                    // Of course this is a dummy middleware, but it's just for testing purposes
                    let future = async move {
                        headers
                            .borrow_mut()
                            .set("Authorization", "Bearer token")
                            .unwrap();
                    };

                    Box::pin(future)
                }),
            ]);

        let abort_controller = web_sys::AbortController::new().unwrap();
        let signal = Rc::new(abort_controller.signal());

        let result = &client
            .get_posts(
                client.prepare_get_posts_url(),
                signal.clone(),
                GetPostsParams::default(),
            )
            .await;

        assert!(result.is_ok());

        let result: Vec<PostBody> =
            deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    async fn test_hook() {
        let t = render!(
            {
                let posts = use_get_posts(GetPostsParams::default());

                use_remember_value((*posts.data).clone());

                html! {
                    <ul>
                        {for (*posts.data).clone().unwrap_or_default().iter().map(|post| html! {
                            <li key={post.id}>
                                <p>{&post.title}</p>
                                <p>{&post.body}</p>
                            </li>
                        })}
                    </ul>
                }
            },
            TestRoot
        )
        .await;

        t.wait_for(1000.0, || {
            let post_items = t.query_all_by_role("listitem");
            post_items.len() == 100
        })
        .await;

        let post_items = t.query_all_by_role("listitem");
        assert_eq!(post_items.len(), 100);

        assert_eq!(
            t.get_remembered_value::<Option<Vec<PostBody>>>()
                .unwrap_or_default()
                .len(),
            100
        );
    }

    #[wasm_bindgen_test]
    async fn test_hook_async() {
        let t = render!(
            {
                let posts = use_get_posts_async();

                use_remember_value((*posts.data).clone());

                if let Some(error) = (*posts.error).clone() {
                    return html! { format!("Error fetching posts: {error:?}") }
                }

                html! {
                    <>
                        <button onclick={Callback::from(move |_event: MouseEvent| {
                            posts.trigger.emit(GetPostsParams::default());
                        })}>{ "Fetch" }</button>

                        {if *posts.loading {
                            html! { "Loading..." }
                        } else {
                            html! {
                                <ul>
                                    {for (*posts.data).clone().unwrap_or_default().iter().map(|post| html! {
                                        <li key={post.id}>
                                            <p>{&post.title}</p>
                                            <p>{&post.body}</p>
                                        </li>
                                    })}
                                </ul>
                            }
                        }}
                    </>
                }
            },
            TestRoot
        )
        .await;

        let button = t.query_by_role("button");

        assert_eq!(t.query_all_by_role("listitem").len(), 0);

        assert_eq!(
            t.get_remembered_value::<Option<Vec<PostBody>>>()
                .unwrap_or_default()
                .len(),
            0
        );

        button.click().await;

        t.wait_for(1000.0, || {
            let post_items = t.query_all_by_role("listitem");
            post_items.len() == 100
        })
        .await;

        assert_eq!(t.query_all_by_role("listitem").len(), 100);

        assert_eq!(
            t.get_remembered_value::<Option<Vec<PostBody>>>()
                .unwrap_or_default()
                .len(),
            100
        );
    }
}
