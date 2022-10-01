//! The `http` DSL provides built-in functions and types to perform declarative
//! assertions against an http response.
//!
//! The following example demonstrates some of the assertions we can run against
//! an http response of a specific endpoint.
//!
//! ```rust
//! use grillon::{Result, Grillon, StatusCode, json};
//! use grillon::dsl::{contains, is, is_less_than, http::is_success};
//! use grillon::header::{HeaderValue, CONTENT_TYPE};
//!
//! #[tokio::test]
//! async fn check_users_endpoint() -> Result<()> {
//!    Grillon::new("http://jsonplaceholder.typicode.com")?
//!        .get("albums/1")
//!        .assert()
//!        .await
//!        .status(is_success())
//!        .headers(contains(vec![
//!             (CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"))
//!         ]))
//!        .json_body(is(json!({
//!             "id": 1,
//!             "title": "quidem molestiae enim",
//!             "userId": 1
//!         })))
//!        .response_time(is_less_than(500));
//!
//!    Ok(())
//! }

mod body;
mod headers;
// mod status;
mod status_bis;
mod time;

pub use self::{
    body::JsonBodyDsl,
    headers::HeadersDsl,
    // status::{is_client_error, is_server_error, is_success, StatusCodeDsl},
    status_bis::*,
    time::TimeDsl,
};
