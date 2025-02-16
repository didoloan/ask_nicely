# ask_nicely

[![Crates.io](https://img.shields.io/crates/v/api_client.svg)](https://crates.io/crates/ask_nicely)
[![Docs.rs](https://docs.rs/ask_nicely/badge.svg)](https://docs.rs/ask_nicely)
<!-- [![Build Status](...)]  and so on... -->

A flexible and easy-to-use API client library

A flexible and easy-to-use API client library for Rust. This crate simplifies interacting with RESTful APIs by providing a structured approach to defining requests, handling authentication, and managing responses.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ask_nicely = "0.1.0" // Replace with the latest version
# Add other dependencies your crate requires, like tokio,serd, etc.
```

## Key Features

* **Declarative Request Definition:** Define API requests as structs, leveraging Rust's type system for compile-time safety and clarity.
* **Flexible Authentication:** Supports various authentication methods, including Basic, Bearer, Digest, and API keys.
* **Easy Request Building:** Provides a builder pattern for constructing API clients with custom configurations like timeouts and redirect policies.
* **Asynchronous Requests:** Built on top of `reqwest` for efficient asynchronous HTTP requests.
* **Customizable Request Data:** Supports sending data in various formats, including JSON, XML, form data, and binary payloads.
* **Error Handling:** Provides a clear error handling mechanism for both network and API-related errors.
* **Macro-Generated Clients:** Simplifies client creation with convenient macros.

## Usage

### 1. Defining Requests

Use the `#[derive(Request)]` macro to define your API requests as structs. Specify the HTTP method, path, authentication requirements, and expected response type.

```rust
use ask_nicely::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Request, Serialize, Deserialize, Debug)]
#[request(
    authed = true,
    response = UserResponse,
    method = "GET",
    path = "/users/{id}",
    get_data = Json
)]
pub struct GetUser {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
}

#[derive(Request, Serialize, Deserialize, Debug)]
#[request(
    authed = false,
    response = Vec<Product>,
    method = "GET",
    path = "/products",
    get_data = Query
)]
pub struct GetProducts {
   pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
}
```

## 2. Creating an API Client
Use the gen_client macro to generate an API client with a specified base URL or an environment variable holding it.

```rust
use api_client::prelude::*;

#[gen_client("https://api.example.com")] // Or: #[gen_client(API_BASE_URL)]
pub struct MyApiClient;
```

## 3. Making Requests
```rust
use ask_nicely::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = Authentication::Bearer("your_api_token"); // Replace with your auth method
    let mut client = MyApiClient::new(auth);

    let user_request = GetUser { id: 123 };
    let user_response = client.request(&user_request).await?;
    // Handle the response...

    let product_request = GetProducts { limit: Some(10) };
    let products_response = client.request(&product_request).await?;
    // Handle the response...

    Ok(())
}
```

## 4. Authentication
The Authentication enum provides several options:

- Basic(username, password)
- Digest(username, password)
- Bearer(token)
- ApiKeys(vec![(key1, value1), (key2, value2)])
- None

## 5. Error Handling
The request method returns a Result<Result<T, ApiError>, Error>. The outer Result handles network-related errors, while the inner Result handles API-specific errors (e.g., 404 Not Found). The ApiError struct contains the status code and response body, allowing for inspection and graceful error handling. Refer to the crate documentation for more details on error handling.