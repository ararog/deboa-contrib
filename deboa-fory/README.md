# deboa-fory

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-fory)](https://crates.io/crates/deboa-fory) [![crates.io](https://img.shields.io/crates/v/deboa-fory?style=flat-square)](https://crates.io/crates/deboa-fory) [![Build Status](https://github.com/deboa-client/deboa-contrib/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/deboa-client/deboa-contrib/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-fory) [![Documentation](https://docs.rs/deboa-fory/badge.svg)](https://docs.rs/deboa-fory/latest/deboa-fory) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/deboa-client/deboa-contrib/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/deboa-client/deboa-fory)

Apache Fory serializer support for Deboa

## Features

- [x] Fory serializer
- [x] Fory deserializer

## Install

Either run from command line:

`cargo add deboa-fory`

Or add to your `Cargo.toml`:

```toml
deboa-fory = "0.0.1"
```

## Usage

```rust, ignore
use deboa::{errors::DeboaError, request::post, Deboa};
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use fory::{Fory, ForyObject};

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

let mut fory = Fory::default();
let _ = fory.register::<Person>(1);

let mut client = Deboa::default();

let person = Person {
    name: "John Doe".to_string(),
    age: 30,
};

let request = post("http://localhost:8080/persons")?
    .body_as_fory(&fory, person)?;

let response: Person = request
    .send_with(&mut client)
    .await?
    .body_as_fory(&fory)
    .await?;
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
