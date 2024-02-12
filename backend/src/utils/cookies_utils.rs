use actix_web::cookie::{time::Duration, Cookie, SameSite};

pub fn cookie_builder<'a, T>(cookie_name: &'a str, cookie_value: T) -> Cookie<'a>
where
    T: Into<String>,
{
    let mut cookie = Cookie::new(cookie_name, cookie_value.into());

    cookie.set_same_site(SameSite::None);
    cookie.set_http_only(true);
    cookie.set_max_age(Duration::days(30));
    cookie.set_path("/");

    cookie
}


pub fn cookie_destroyer<'a>(cookie_name: &'a str) -> Cookie<'a> {
    let mut cookie = Cookie::new(cookie_name, "");

    cookie.make_removal();

    cookie
}