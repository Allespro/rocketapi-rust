## Introduction

This library provides a pure Rust interface for the RocketAPI.

## Installing

To install rocketapi add in Cargo.toml
```bash

```

## Examples

InstagramAPI example
```rust
use rocketapi::instagramapi::InstagramAPI;
use rocketapi::errors::RocketAPIError;

#[tokio::main]
async fn main() {
    let mut instagram_api: InstagramAPI = InstagramAPI::new(
        "Your API key".to_string(),
        std::time::Duration::from_secs(30)
    );
    let username:&str = "kanyewest";
    match instagram_api.get_user_info(username).await {
        Ok(result) => {
            println!("Response: {:?}", result)
        }
        Err(RocketAPIError::BadResponse(msg)) => println!("{}", msg),
        Err(RocketAPIError::NotFound(msg)) => println!("{}", msg),
        Err(RocketAPIError::RequestError(msg)) => println!("{}", msg),
    }
}
```

ThreadsAPI example
```rust
use rocketapi::threadsapi::ThreadsAPI;
use rocketapi::errors::RocketAPIError;

#[tokio::main]
async fn main() {
    let mut threads_api: ThreadsAPI = ThreadsAPI::new(
        "Your API key".to_string(),
        std::time::Duration::from_secs(30)
    );
    let user_id: u64 = 65107478842;
    match threads_api.get_user_feed(&user_id, None).await {
        Ok(result) => {
            println!("Response: {:?}", result)
        }
        Err(RocketAPIError::BadResponse(msg)) => println!("{}", msg),
        Err(RocketAPIError::NotFound(msg)) => println!("{}", msg),
        Err(RocketAPIError::RequestError(msg)) => println!("{}", msg),
    }
}
```

## Usage

See the [documentation](https://docs.rocketapi.io) for more information.
