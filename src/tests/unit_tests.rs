mod unit_tests {
    use crate::hello_world;
    use actix_web::http;

    #[actix_web::test]
    async fn test_hello_world_ok() {
        let resp = hello_world().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
