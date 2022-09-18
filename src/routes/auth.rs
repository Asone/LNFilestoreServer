use rocket::{
    form::{Form, Strict},
    http::{Cookie, CookieJar, SameSite, Status},
};

use crate::{
    db::{models::user_token::UserToken, PostgresConn},
    forms::login_user::LoginUser,
};

use std::{env};

/// Authentication route
#[rocket::post("/auth", data = "<user_form>")]
pub async fn login(
    db: PostgresConn,
    cookies: &CookieJar<'_>,
    user_form: Form<Strict<LoginUser>>,
) -> rocket::http::Status {
    let user = user_form.into_inner().into_inner();

    let session = user.login(db).await;

    match session {
        Ok(user_session) => {
            let token = UserToken::generate_token(user_session).unwrap();
            let cookie = Cookie::build("session", token)
                .same_site(same_site_cookie())
                .secure(secure_cookie())
                .finish();

            cookies.add(cookie);
            Status::Ok
        }
        Err(_) => Status::ExpectationFailed,
    }
}

fn same_site_cookie() -> SameSite {
    let cookie_same_site_policy = env::var("COOKIES_SAME_SITE_POLICY");

    match cookie_same_site_policy {
        Ok(value) => {
            match value.as_str(){
                "lax" => SameSite::Lax,
                "none" => SameSite::None,
                "strict" => SameSite::Strict,
                _ => SameSite::Lax
            }
        }
        Err(_) => SameSite::Lax
}

}
/// Secures cookie based on environment
fn secure_cookie() -> bool {
    let cookie_is_secure = env::var("COOKIES_IS_SECURE_POLICY_POLICY");

    match cookie_is_secure {
        Ok(value) => {
            value.parse::<bool>().unwrap_or(false)
        },
        Err(_) => false
    }
}
