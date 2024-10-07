use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::{set_token_cookies, AUTH_TOKEN};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use crate::crypt::token::{validate_token, Token};
use crate::model::user::{UserBmc, UserForAuth};

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

	if ctx_ext_result.is_err()
		&& !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN));
	}

	req.extensions_mut().insert(ctx_ext_result);

	Ok(next.run(req).await)
}

async fn _ctx_resolve(
	mm: State<ModelManager>,
	cookies: &Cookies
) -> CtxExtResult {
	// -- Get token string
	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(CtxExtError::TokenNotInCookie)?;

	// -- Parse token
	let token: Token = token.parse()
		.map_err(|_| CtxExtError::TokenWrongFormat)?;

	// -- Get UserForAuth
	let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
		.await
		.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
		.ok_or(CtxExtError::UserNotFound)?;

	// -- Validate Token
	validate_token(&token, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::FailValidate)?;

	// -- Update Token
	set_token_cookies(cookies, &user.username, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::CannotSetTokenCookie)?;

	// -- Create CtxExtResult
	Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!(" {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	TokenNotInCookie,
	TokenWrongFormat,
	FailValidate,
	CannotSetTokenCookie,

	UserNotFound,
	ModelAccessError(String),

	CtxNotInRequestExt,
	CtxCreateFail(String),
}

impl core::fmt::Display for CtxExtError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

// endregion: --- Ctx Extractor Result/Error
