use graphql::ErrorExtensions;
use serde::Serialize;

pub type GqlResult<T, E = ()> = Result<T, GqlError<E>>;

#[derive(graphql::SimpleObject)]
#[graphql(name = "Ok")]
pub struct GqlOk {
    ok: bool,
}

impl GqlOk {
    pub fn new() -> Self {
        Self { ok: true }
    }
}

const EXTENDED_ERROR: &str = "Extended Error";
const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

pub enum GqlError<E: Serialize = ()> {
    Extended(E),
    InternalServerError,
}

impl<E: Serialize> Into<graphql::Error> for GqlError<E> {
    fn into(self) -> graphql::Error {
        match self {
            GqlError::Extended(ref error) => {
                graphql::Error::new(EXTENDED_ERROR).extend_with(|_, ext| {
                    if let Ok(reason) = graphql::to_value(error) {
                        ext.set("details", reason)
                    }
                })
            }
            GqlError::InternalServerError => graphql::Error::new(INTERNAL_SERVER_ERROR),
        }
    }
}

#[macro_export]
macro_rules! gql_error {
    ($error:item) => {
        #[derive(serde::Serialize, Copy, Clone)]
        #[serde(
            tag = "reason",
            content = "payload",
            rename_all = "SCREAMING_SNAKE_CASE"
        )]
        $error
    };
}
