use serde::{Deserialize, Serialize};
use wasm_bindgen_test::wasm_bindgen_test;
use yew::prelude::*;
use yewlish_fetch_macro::FetchSchema;
use yewlish_testing_tools::*;

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

#[derive(FetchSchema)]
enum Test {
    #[get("/todos/{id}", slugs = TodoSlug, res = Todo)]
    Todo,
    #[get("/todos", res = Vec<Todo>)]
    Todos,
    #[post("/todos", body = CreateTodo, res = Todo)]
    CreateTodo,
    #[put("/todos/{id}", slugs = TodoSlug, body = UpdateTodo, res = Todo)]
    UpdateTodo,
    #[delete("/todos/{id}", slugs = TodoSlug, res = String)]
    DeleteTodo,
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, Clone, PartialEq)]
struct TestRootProps {
    children: Children,
}

#[function_component(TestRoot)]
fn test_root(props: &TestRootProps) -> Html {
    let client = TestFetchClient::new("http://127.0.0.1:5000".to_string());

    html! {
        <TestFetchClientProvider client={client}>
            <div class="test-root">
                {for props.children.iter()}
            </div>
        </TestFetchClientProvider>
    }
}

#[wasm_bindgen_test]
async fn test_fetch_schema_client() {
    let client = TestFetchClient::new("http://127.0.0.1:5000".to_string());
    let result = client.todos(TodosParams::default()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);

    let result = client
        .create_todo(CreateTodoParams {
            body: CreateTodo {
                title: "Test".to_string(),
                description: None,
            },
            ..Default::default()
        })
        .await;

    assert!(result.is_ok());
    let todo = result.unwrap();

    assert_eq!(todo.title, "Test".to_string());
    assert_eq!(todo.description, None);
    assert!(todo.done == 0);

    let result = client.todos(TodosParams::default()).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], todo);

    let result = client
        .todo(TodoParams {
            slugs: TodoSlug { id: todo.id },
            ..Default::default()
        })
        .await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, todo);

    let result = client
        .update_todo(UpdateTodoParams {
            slugs: TodoSlug { id: todo.id },
            body: UpdateTodo {
                title: Some("Test2".to_string()),
                description: Some("Test".to_string()),
                done: Some(1),
            },
            ..Default::default()
        })
        .await;

    assert!(result.is_ok());
    let todo = result.unwrap();
    assert_eq!(todo.title, "Test2".to_string());
    assert_eq!(todo.description, Some("Test".to_string()));
    assert!(todo.done == 1);

    let result = client.todos(TodosParams::default()).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], todo);

    let result = client
        .delete_todo(DeleteTodoParams {
            slugs: TodoSlug { id: todo.id },
            ..Default::default()
        })
        .await;

    assert!(result.is_ok());

    let result = client.todos(TodosParams::default()).await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.len(), 0);
}

// #[wasm_bindgen_test]
// async fn test_fetch_schema_get_todos() {
//     let t = render!(
//         {
//             let res = use_todos(TodosParams::default());
//             use_remember_value(res);

//             html! {}
//         },
//         TestRoot
//     )
//     .await;

//     assert!(
//         t.wait_for(2000, || t.get_state::<UseTodosHandle>().data.is_some())
//             .await
//     );

//     assert_eq!(*t.get_state::<UseTodosHandle>().data, Some(vec![]));
// }