# Deboa Contrib

## Description

**deboa-contrib** contains crates with additional functionality for the deboa HTTP client.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information. Keep in mind API for prior to 0.1.0 is subject to change. Proper deprecation will be added in the next stable release.

## Install

```toml
deboa-extras = { version = "0.0.9" }
```

## Usage

```rust
use deboa::{
    request::{DeboaRequest, FetchWith, get},
    Result,
};
use deboa_tokio::Client;
use deboa_extras::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new Client instance, set timeouts, catches and protocol.
  let client = Client::new();

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
    .header(header::CONTENT_TYPE, "application/json")
    .send_with(&client)
    .await?
    .body_as(JsonBody)
    .await?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## Subprojects

### [deboa-extras](https://github.com/ararog/deboa/tree/develop/deboa-extras)

Pluggable compression/decompression, serializers, sse, websockets and catchers.
All of them are optional. This is the place to contribute with your own pluggable features.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
