use deboa::request::get;
use deboa_extras::serde::json::JsonBody;
use deboa_hickory::HickoryDnsResolver;
use deboa_tokio::CustomClient;
use http::header;

#[derive(serde::Deserialize, Debug)]
struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_lookup() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Client instance, set timeouts, catches and protocol.
    let dns_resolver = HickoryDnsResolver;
    let client = CustomClient::<HickoryDnsResolver>::builder()
        .dns_resolver(dns_resolver)
        .build();

    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
        .header(header::CONTENT_TYPE, "application/json")
        .send_with(&client)
        .await?
        .body_as(JsonBody)
        .await?;

    assert_eq!(posts.len(), 100);

    Ok(())
}
