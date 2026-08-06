//! A hook that redirects requests to another hook.
//!
//! This is useful for forwarding requests to other hooks.
//!
//! # Example
//!
//! ```
//! use deboa::{DeboaRequest, DeboaResponse};
//! use deboa_extras::hook::RedirectHook;
//! use tackle::Hook;
//!
//! struct MyHook;
//!
//! impl Hook<DeboaRequest, DeboaResponse> for MyHook {
//!     type Result = Result<DeboaResponse>;
//!     type Error = deboa::errors::DeboaError;
//!
//!     async fn call(&self, input: DeboaRequest) -> Self::Result {
//!         Ok(DeboaResponse::new("Hello, world!"))
//!     }
//! }
//!
//! let hook = RedirectHook::new(MyHook);
//! ```
use deboa::{
    errors::{DeboaError, RequestError},
    request::DeboaRequest,
    response::DeboaResponse,
    Result,
};
use tackle::{Chain, Hook};

/// A hook that redirects requests to another hook.
pub struct RedirectHook<H> {
    inner: H,
    limit: usize,
}

impl<H> RedirectHook<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>, Error = DeboaError>,
{
    /// Creates a new redirect hook.
    pub fn new(inner: H, limit: usize) -> Self {
        Self { inner, limit }
    }
}

impl<H> Hook<DeboaRequest, DeboaResponse> for RedirectHook<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>, Error = DeboaError>,
{
    type Result = Result<DeboaResponse>;
    type Error = DeboaError;

    async fn call(&self, input: DeboaRequest) -> Self::Result {
        let mut request = input;
        let mut redirect_count = 0;

        loop {
            let result = self
                .inner
                .call(request)
                .await?;

            if result
                .status()
                .is_redirection()
            {
                // Check if we should redirect
                if let Some(location) = result
                    .headers()
                    .get("Location")
                {
                    if redirect_count >= self.limit {
                        return Err(DeboaError::Request(RequestError::Send {
                            message: "Redirect limit exceeded".to_string(),
                        }));
                    }

                    let location_str = location
                        .to_str()
                        .map_err(|_| DeboaError::Header {
                            message: "Invalid location header".to_string(),
                        })?;

                    // Create a new request with the redirected URL, deconstructing the previous one
                    request = DeboaRequest::at(location_str, http::Method::GET)
                        .map_err(|e| {
                            DeboaError::Request(RequestError::Parse { message: e.to_string() })
                        })?
                        .build()?;
                    redirect_count += 1;
                } else {
                    return Ok(result);
                }
            } else {
                return Ok(result);
            }
        }
    }
}

/// A hook that redirects requests to a chain of hooks.
pub struct Redirect {
    limit: usize,
}

impl Default for Redirect {
    fn default() -> Self {
        Self { limit: 10 }
    }
}

impl Redirect {
    /// Creates a new redirect hook.
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }
}

impl<H> Chain<H, DeboaError, DeboaRequest, DeboaResponse> for Redirect
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>, Error = DeboaError>,
{
    type Hook = RedirectHook<H>;

    fn chain(&self, hook: H) -> Self::Hook {
        RedirectHook::new(hook, self.limit)
    }
}
