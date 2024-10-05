use yewlish_query::QuerySchema;

#[derive(QuerySchema)]
enum Test {
    #[get(path = "/test")]
    Variant1,
    #[get(path = "/test2")]
    Variant2,
}

#[test]
fn test_query_schema() {
    let client = TestQueryClient {};

    assert_eq!(client.get_queryable(Test::Variant1).query(), "/test");
    assert_eq!(client.get_queryable(Test::Variant2).query(), "/test2");
}
