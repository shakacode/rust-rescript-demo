pub mod http;
pub mod inputs;
pub mod schema;

#[macro_use]
mod db;
#[macro_use]
mod result;

mod mutations;
mod queries;

use result::{GqlError, GqlOk, GqlResult};
