use caramelo::{expect, matchers::eq};
use deboa::{request::get, TestResult};
use deboa_extras::hook::redirect::Redirect;
use deboa_tokio::Client;

#[tokio::test]
async fn test_redirect() -> TestResult<()> {
    let client = Client::default().chain(Redirect::default());

    let response = get("https://httpbin.org/redirect/1")?
        .send_with(&client)
        .await?;

    expect(response.status()).to_be(eq(200));

    Ok(())
}
