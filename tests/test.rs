#[tokio::test]
async fn test_root() {
    let hc = httpc_test::new_client("http://127.0.0.1:3000").unwrap();

    let status = hc.do_get("/").await.unwrap().status();
    assert_eq!(status, 200);
}
