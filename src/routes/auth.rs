use rocket::{
    form::{Form, Strict},
    http::{Cookie, CookieJar, SameSite, Status},
};

use crate::{
    db::{models::user_token::UserToken, PostgresConn},
    forms::login_user::LoginUser,
};

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
                .same_site(SameSite::None)
                .secure(true)
                .finish();

            cookies.add(cookie);
            Status::Ok
        }
        Err(_) => Status::ExpectationFailed,
    }
}
