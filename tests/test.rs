#[tokio::test]
async fn create_ticket() {
    let hc = httpc_test::new_client("http://127.0.0.1:3000").unwrap();

    let status = hc.do_post("/tickets", r#""#).await.unwrap().status();
    assert_eq!(status, 200);
}

#[tokio::test]
async fn get_tickets() {
    let hc = httpc_test::new_client("http://127.0.0.1:3000").unwrap();

    let status = hc.do_get("/tickets").await.unwrap().status();
    assert_eq!(status, 200);
}

#[tokio::test]
async fn root_should_work() {
    let hc = httpc_test::new_client("http://127.0.0.1:3000").unwrap();

    let status = hc.do_get("/").await.unwrap().status();
    assert_eq!(status, 200);
}

#[tokio::test]
async fn login_should_work() {
    let hc = httpc_test::new_client("http://127.0.0.1:3000").unwrap();

    let req_login = hc
        .do_post(
            "/api/login",
            (
                r#"username=dev&password=password"#,
                "application/x-www-form-urlencoded",
            ),
        )
        .await
        .unwrap();

    assert_eq!(req_login.status(), 200);

    let req_login_fail = hc
        .do_post(
            "/api/login",
            (
                r#"username=dev&password=wrong"#,
                "application/x-www-form-urlencoded",
            ),
        )
        .await
        .unwrap();

    assert_ne!(req_login_fail.status(), 200);
}
