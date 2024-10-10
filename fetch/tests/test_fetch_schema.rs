use schema::{
    use_todos_with_options, CreateTodoParams, DeleteTodoParams, TestFetchClient,
    TestFetchClientProvider, TodoParams, TodosOptions, TodosParams, UpdateTodoParams,
};
use serde::{Deserialize, Serialize};
use serial_test::serial;
use std::{rc::Rc, sync::Arc};
use wasm_bindgen_test::wasm_bindgen_test;
use yew::prelude::*;
use yewlish_fetch_utils::*;
use yewlish_testing_tools::*;

#[derive(Default, Serialize, PartialEq, Clone, Debug)]
struct TodosQuery {
    done: Option<u8>,
}

#[derive(Default, Deserialize, PartialEq, Clone, Debug)]
struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    done: u8,
}

#[derive(Default, Serialize, PartialEq, Clone)]
struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Default, Serialize, PartialEq, Clone)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    done: Option<u8>,
}

#[derive(Default, Serialize, PartialEq, Clone)]
struct TodoSlug {
    id: u32,
}

mod schema {
    use yewlish_fetch::FetchSchema;

    #[derive(FetchSchema)]
    pub enum Test {
        #[get("/todos/{id}", slugs = TodoSlug, res = Todo)]
        Todo,
        #[get("/todos", query = TodosQuery, res = Vec<Todo>)]
        Todos,
        #[post("/todos", body = CreateTodo, res = Todo)]
        CreateTodo,
        #[put("/todos/{id}", slugs = TodoSlug, body = UpdateTodo, res = Todo)]
        UpdateTodo,
        #[delete("/todos/{id}", slugs = TodoSlug, res = String)]
        DeleteTodo,
    }
}

pub use schema::Test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, Clone, PartialEq)]
struct TestRootProps {
    children: Children,
}

#[function_component(TestRoot)]
fn test_root(props: &TestRootProps) -> Html {
    let client = TestFetchClient::new("http://127.0.0.1:5000").with_middlewares(vec![Arc::new(
        Box::new(|_, headers| {
            headers.set("Authorization", "Bearer token").unwrap();
            headers.set("Content-Type", "application/json").unwrap();
        }),
    )]);

    html! {
        <TestFetchClientProvider client={client}>
            <div class="test-root">
                {for props.children.iter()}
            </div>
        </TestFetchClientProvider>
    }
}

#[wasm_bindgen_test]
#[serial]
async fn test_fetch_schema_client() {
    let client = TestFetchClient::new("http://127.0.0.1:5000").with_middlewares(vec![Arc::new(
        Box::new(|_, headers| {
            headers.set("Authorization", "Bearer token").unwrap();
            headers.set("Content-Type", "application/json").unwrap();
        }),
    )]);

    let abort_controller = web_sys::AbortController::new().unwrap();
    let signal = Rc::new(abort_controller.signal());

    let result = &client
        .todos(
            client.prepare_todos_url(),
            signal.clone(),
            TodosParams::default(),
        )
        .await;

    assert!(result.is_ok());
    let result: Vec<Todo> = deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
    assert_eq!(result, vec![]);

    let result = client
        .create_todo(
            client.prepare_create_todo_url(),
            signal.clone(),
            CreateTodoParams {
                body: CreateTodo {
                    title: "Test".to_string(),
                    description: None,
                },
                ..Default::default()
            },
        )
        .await;

    assert!(result.is_ok());
    let todo: Todo = deserialize_response(result.unwrap().as_str()).unwrap();

    assert_eq!(todo.title, "Test".to_string());
    assert_eq!(todo.description, None);
    assert!(todo.done == 0);

    let result = &client
        .todos(
            client.prepare_todos_url(),
            signal.clone(),
            TodosParams::default(),
        )
        .await;

    assert!(result.is_ok());
    let result: Vec<Todo> = deserialize_response(result.as_ref().unwrap().as_str()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], todo);

    let result = client
        .todo(
            client.prepare_todo_url(),
            signal.clone(),
            TodoParams {
                slugs: TodoSlug { id: todo.id },
                ..Default::default()
            },
        )
        .await;

    assert!(result.is_ok());
    let result: Todo = deserialize_response(result.unwrap().as_str()).unwrap();
    assert_eq!(result, todo);

    let result = client
        .update_todo(
            client.prepare_update_todo_url(),
            signal.clone(),
            UpdateTodoParams {
                slugs: TodoSlug { id: todo.id },
                body: UpdateTodo {
                    title: Some("Test2".to_string()),
                    description: Some("Test".to_string()),
                    done: Some(1),
                },
                ..Default::default()
            },
        )
        .await;

    assert!(result.is_ok());
    let todo: Todo = deserialize_response(result.unwrap().as_str()).unwrap();
    assert_eq!(todo.title, "Test2".to_string());
    assert_eq!(todo.description, Some("Test".to_string()));
    assert!(todo.done == 1);

    let result = client
        .todos(
            client.prepare_todos_url(),
            signal.clone(),
            TodosParams::default(),
        )
        .await;

    assert!(result.is_ok());
    let result: Vec<Todo> = deserialize_response(result.unwrap().as_str()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], todo);

    let result = client
        .delete_todo(
            client.prepare_delete_todo_url(),
            signal.clone(),
            DeleteTodoParams {
                slugs: TodoSlug { id: todo.id },
                ..Default::default()
            },
        )
        .await;

    assert!(result.is_ok());

    let result = client
        .todos(client.prepare_todos_url(), signal, TodosParams::default())
        .await;

    assert!(result.is_ok());

    let result: Vec<Todo> = deserialize_response(result.unwrap().as_str()).unwrap();
    assert_eq!(result.len(), 0);
}

#[wasm_bindgen_test]
#[serial]
pub async fn test_hooks() {
    let t = render!(
        {
            let query = use_todos_with_options(TodosParams::default(), TodosOptions::default());
            use_remember_value(query.data);

            html! {}
        },
        TestRoot
    )
    .await;

    assert_eq!(t.get_state::<Vec<Todo>>().len(), 0);
}
