use crate::ctx::Context;
use crate::models::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, Extension};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth<B>(
    ctx: Result<Context>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_async_resolver<B>(
    _mc: Extension<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_async_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // get user_id
            match _mc.get_username(user_id.try_into().unwrap()).await {
                Ok(username) => Ok(Context::new(user_id.try_into().unwrap(), &username)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong
    //
    if result_ctx.is_err() && matches!(result_ctx, Err(Error::AuthFailNoAuthToken)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

/// Parse a token of format user-[user-id].[expiration].[signature]
fn parse_token(token: String) -> Result<(u32, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u32 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
