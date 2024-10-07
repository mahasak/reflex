use std::fmt::{Display, Formatter};
use std::str::FromStr;
use axum::body::HttpBody;
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent, Error, Result};
use crate::utils::{b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc};

// region:  --- Token Type
// String format : `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
    pub ident: String,  // Identifier (username e.g.)
    pub exp: String,    // Expiration date in RFC3339
    pub sign_b64u: String,   // Signature, encoded
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split(".").collect();
        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat)
        }

        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode(ident_b64u)
                .map_err(|_| Error::TokenCannotDecodeIdent)?,
            exp: b64u_decode(exp_b64u)
                .map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{}.{}.{}",
               b64u_encode(&self.ident),
               b64u_encode(&self.exp),
               self.sign_b64u
        )
    }
}

// endregion:  --- Token Type

// region:  --- Token Generation and Validation
pub fn generate_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();

    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_token(original_token: &Token, salt: &str) -> Result<()> {
    let config = &config();

    _validate_token_sign_and_exp(original_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}
// endregion:  --- Token Generation and Validation

// region:  --- (private) Token Generation and Validation
fn _generate_token(
    ident: &str,
    duration_sec: f64,
    salt : &str,
    key: &[u8]
) -> Result<Token> {
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // -- sign
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt,key)?;

    Ok(Token { ident, exp, sign_b64u })
}


fn _validate_token_sign_and_exp(
    original_token: &Token,
    salt: &str,
    key: &[u8]
) -> Result<()>{
    // Validate
    let new_sign_b64_u = _token_sign_into_b64u(&original_token.ident, &original_token.exp, salt,key)?;

    if new_sign_b64_u != original_token.sign_b64u {
        return Err(Error::TokenSignatureNotMatching)
    }

    // Validate expiration
    let origin_exp = parse_utc(&original_token.exp).map_err(|_| Error::TokenExpNotIso)?;

    let now = now_utc();
    if origin_exp < now {
        return Err(Error::TokenExpired)
    }

    Ok(())
}

fn _token_sign_into_b64u(
    ident: &str,
    exp: &str,
    salt: &str,
    key: &[u8]
) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
    let signature = encrypt_into_b64u(
        key,
        &EncryptContent {
            content,
            salt: salt.to_string(),
        }
    )?;

    Ok(signature)
}
// endregion:  --- (private) Token Generation and Validation

// region: --- Tests
#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;
    use anyhow::Result;
    use dotenv::dotenv;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        // -- Fixture
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2024-10-09T16:00:00Z".to_string(),
            sign_b64u: "fx-sign".to_string(),
        };
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNC0xMC0wOVQxNjowMDowMFo.fx-sign";

        // -- Exec & Check
        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        // -- Fixture
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2024-10-09T16:00:00Z".to_string(),
            sign_b64u: "fx-sign".to_string(),
        };
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNC0xMC0wOVQxNjowMDowMFo.fx-sign";

        // -- Exec
        let token: Token = fx_token_str.parse()?;

        // -- Check
        // assert_eq!(token, fx_token);
        assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

        Ok(())
    }

    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        // -- Fixture
        dotenv().ok();
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration = 0.02;  // 20 ms
        let token_key = &config().TOKEN_KEY;

        let fx_token = _generate_token(fx_user, fx_duration, fx_salt, token_key)?;

        // -- Exec
        thread::sleep(Duration::from_millis(10));
        let res = validate_token(&fx_token, fx_salt);

        // -- Check
        res?;

        Ok(())
    }

    #[test]
    fn test_validate_web_token_expired() -> Result<()> {
        // -- Fixture
        dotenv().ok();
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration = 0.01;  // 20 ms
        let token_key = &config().TOKEN_KEY;

        let fx_token = _generate_token(fx_user, fx_duration, fx_salt, token_key)?;

        // -- Exec
        thread::sleep(Duration::from_millis(20));
        let res = validate_token(&fx_token, fx_salt);

        // -- Check
        assert!(res.is_err());
        assert!(
            matches!(res, Err(Error::TokenExpired)),
            "should have matched `Err(Error::TokenExpired` but was `{res:?}`"
        );

        Ok(())
    }
}
// endregion: --- Tests