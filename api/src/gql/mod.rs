pub mod http;
pub mod inputs;
pub mod schema;

#[macro_use]
mod db;

mod mutations;
mod queries;
mod result;

use result::{GqlError, GqlOk, GqlResult};
