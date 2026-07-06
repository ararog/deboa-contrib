use deboa::request::get;
use deboa_extras::serde::json::JsonBody;
use deboa_tokio::Client;
use http::header;

#[derive(serde::Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::test]
async fn test_lookup() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Client instance, set timeouts, catches and protocol.
    let dns_resolver = deboa_hickory::HickoryDnsResolver;
    let client = Client::builder()
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
