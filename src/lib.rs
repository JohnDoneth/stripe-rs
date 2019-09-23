// Copyright 2019 Wyyerd Group, LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/stripe-rust/")]
#![recursion_limit = "128"]

//! This crate provides Rust bindings to the Stripe HTTP API.
//!
//! ## Getting Started
//!
//! To get started, we need to create a client:
//!
//! ```rust
//! let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! ```
//!
//! Then we can begin making requests as we'd like.  Most Stripe requests accept
//! many optional parameters, so we usually get the `::new(...)` with any required
//! params and then set the ones we want from there.
//!
//! Most requests for creating or updating a Stripe object use the same Rust struct,
//! so you may frequently need to refer to the [official API docs](https://stripe.com/docs/api)
//! to determine which fields are required for either request.
//!
//! ```rust,no_run
//! /* Creating a Stripe Charge */
//!
//! # let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! let token = "tok_ID_FROM_CHECKOUT".parse().unwrap();
//! let mut params = stripe::CreateCharge::new();
//! // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
//! params.amount = Some(1095); // e.g. $10.95
//! params.source = Some(stripe::ChargeSourceParams::Token(token));
//!
//! // Example: Override currency to be in Canadian Dollars
//! params.currency = Some(stripe::Currency::CAD);
//! let charge = stripe::Charge::create(&client, params).unwrap();
//! println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
//! ```
//!
//! ```rust,no_run
//! /* Listing Stripe Charges */
//!
//! # let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! let params = stripe::ListCharges::new();
//! let charges = stripe::Charge::list(&client, params).unwrap();
//! println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
//! ```

#![allow(clippy::map_clone)]
// N.B. not sure if this rule will break compatibility with older rust versions we might want to support
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::large_enum_variant)]

mod client;
mod error;
mod ids;
mod params;
mod resources;

pub use crate::client::Client;
pub use crate::error::{Error, ErrorCode, ErrorType, RequestError, WebhookError};
pub use crate::ids::*;
pub use crate::params::{
    Expandable, Headers, List, Metadata, Object, RangeBounds, RangeQuery, Timestamp,
};
pub use crate::resources::*;

/*#[cfg(not(feature = "async"))]
mod config {
    use crate::error::Error;

    /// An alias for `Result`.
    ///
    /// ```rust,ignore
    /// type Response<T> = Result<T, Error>;
    /// ```
    ///
    /// If the `async` feature is enabled, this type is redefined as:
    ///
    /// ```rust,ignore
    /// type Response<T> = Box<dyn Future<Item = T, Error = Error> + Send>
    /// ```
    pub type Response<T> = Result<T, Error>;

    pub(crate) type Client = crate::client::Client;

    #[inline]
    pub fn ok<T>(ok: T) -> Response<T> {
        Ok(ok)
    }

    #[inline]
    pub fn err<T>(err: Error) -> Response<T> {
        Err(err)
    }
}*/


mod config {
    use crate::error::Error;
    
    use std::future::Future;
    use futures_util::future;
    
    pub type Response<T> = Result<T, Error>;
    //pub type Response<T> = Box<dyn Future<Output = ResponseResult<T>> + Send>;

    pub(crate) type Client = crate::client::Client;

    #[inline]
    pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
        Box::new(future::ready(Ok(ok)))
    }

    #[inline]
    pub(crate) fn err<T: Send + 'static>(err: Error) -> Response<T> {
        Box::new(future::ready(Err(err)))
    }
}

// N.B. export for doc purposes
pub use self::config::Response;
