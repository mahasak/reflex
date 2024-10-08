use crate::{crypt, model, utils, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;
use crate::web::Error::LoginFailUsernameNotFound;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Login
	LoginFail,
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd(i64),
	LoginFailPwdNotMatching(i64),

	// -- Rpc
	RpcMethodUnknown(String),
	RpcMissingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),
	// -- Modules
	Model(model::Error),
	Crypt(crypt::Error),

	// -- External Modules
	SerdeJson(String),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!(" {:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}
// endregion: --- Axum IntoResponse
impl From<model::Error> for Error {
	fn from(e: model::Error) -> Self {
		Self::Model(e)
	}
}



impl From<web::mw_auth::CtxExtError> for Error {
	fn from(e: web::mw_auth::CtxExtError) -> Self {
		Self::CtxExt(e)
	}
}

impl From<crypt::Error> for Error {
	fn from(e: crypt::Error) -> Self {
		Self::Crypt(e)
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Self::SerdeJson(e.to_string())
	}
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		#[allow(unreachable_patterns)]
		match self {
			// Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd(..)
			| LoginFailPwdNotMatching(..) => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			},

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Model
			Model(model::Error::EntityNotFound { entity, id}) =>
				(
					StatusCode::BAD_REQUEST,
					ClientError::ENTITY_NOT_FOUND { entity, id: id.clone() }
				),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	ENTITY_NOT_FOUND {entity: &'static str, id: i64},
	SERVICE_ERROR,
}
// endregion: --- Client Error
