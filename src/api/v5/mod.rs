use crate::impl_serde_from_str;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub mod funding_account;
pub mod model;
pub mod orderbook_trading;
pub mod public_data;
pub mod trading_account;

pub use self::model::*;
// re-export funding_account module
pub use self::funding_account::bill::*;
pub use self::funding_account::currencies::*;
pub use self::funding_account::deposit::*;
pub use self::funding_account::transfer::*;
pub use self::funding_account::withdrawal::*;
// re-export trading_account module
pub use self::trading_account::*;
// re-export public data module
pub use self::public_data::*;
// re-export trading module
pub use self::orderbook_trading::fill::*;
pub use self::orderbook_trading::market_data::*;
pub use self::orderbook_trading::orders::*;

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const AUTH: bool = false;

    type Response: DeserializeOwned + Debug;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(deserialize_with = "crate::serde_util::deserialize_from_str")]
    pub code: u32,
    pub msg: String,
    pub data: Option<T>,
}