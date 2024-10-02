use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64},
	// -- Modules
	Store(store::Error),
	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

}

impl From<store::Error> for Error {
	fn from(e: store::Error) -> Self {
		Self::Store(e)
	}
}

impl From<sqlx::Error> for Error {
	fn from(e: sqlx::Error) -> Self {
		Self::Sqlx(e)
	}
}