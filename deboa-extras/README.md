# Deboa Extras

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-extras)](https://crates.io/crates/deboa-extras) [![crates.io](https://img.shields.io/crates/v/deboa-extras?style=flat-square)](https://crates.io/crates/deboa-extras) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-extras) [![Documentation](https://docs.rs/deboa-extras/badge.svg)](https://docs.rs/deboa-extras/latest/deboa-extras) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa-extras)

This crate provides additional features for Deboa like compression and serialization.

## Install

Either run from command line:

`cargo add deboa-extras`

Or add to your `Cargo.toml`:

```toml
deboa-extras = "0.0.1"
```

## Features

- `json` serialization
- `msgpack` serialization
- `xml` serialization
- `sse` support

## Usage

### Serialization

```rust, ignore
use deboa::{Deboa, errors::DeboaError, request::post};
use deboa_extras::http::serde::json::JsonBody;

let client = Deboa::default();

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = post("https://jsonplaceholder.typicode.com/posts/1")?
  .body_as(JsonBody, data)?
  .send_with(client)
  .await?;

println!("Response Status Code: {}", response.status());
```

### SSE

```rust, ignore
use deboa::{Deboa, Result};
use deboa_extras::http::sse::response::{IntoEventStream};

let client = Deboa::default();

let response = client.execute("https://sse.dev/test").await?.into_event_stream();

// Poll events, until the connection is closed
// please note that this is a blocking call
while let Some(event) = response.next().await {
    println!("event: {}", event);
}

println!("Connection closed");
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
