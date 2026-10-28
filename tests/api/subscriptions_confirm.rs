use crate::helpers::spawn_app;

#[tokio::test]
async fn it_rejects_confirmations_without_token_with_400() {
    let test_app = spawn_app().await;

    let response = reqwest::get(&format!("{}/api/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}
