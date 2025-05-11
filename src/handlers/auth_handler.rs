use actix_web::{HttpRequest, HttpResponse, web};
use log::info;
use std::sync::Arc;

use crate::{
    constants::COOKIE_NAME,
    services::user_service::UserService,
    types::{
        requests::auth::{login_request::LoginRequest, register_request::RegisterRequest},
        responses::api_response::{ApiResponse, ErrorDetails},
    },
    utils::{
        auth_utils::generate_cookie,
        locale_utils::{Messages, get_lang},
        validation_utils::{
            handle_internal_error, handle_validation_error, validate_login_data,
            validate_register_data,
        },
    },
};

pub async fn register_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    new_user: web::Json<RegisterRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = new_user.into_inner();

    if let Err(errs) = validate_register_data(&data, &messages) {
        let err_msg =
            messages.get_auth_message("register.invalid_data", "Invalid registration data");
        return handle_validation_error(errs, &err_msg);
    }

    match user_service.register_user(data, &messages).await {
        Ok(user) => HttpResponse::Created().json(ApiResponse::success(
            messages.get_auth_message("register.success", "User successfully created."),
            user,
        )),
        Err(err) => handle_internal_error(err),
    }
}

pub async fn jwt_login_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    credentials: web::Json<LoginRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = credentials.into_inner();

    if let Err(errs) = validate_login_data(&data, &messages) {
        let err_msg =
            messages.get_auth_message("login.invalid_credentials", "Invalid login credentials");
        return handle_validation_error(errs, &err_msg);
    }

    match user_service
        .authenticate_user(&data.email, &data.password, &messages)
        .await
    {
        Ok((user, token)) => {
            info!("User {} successfully logged in.", data.email);
            let cookie = generate_cookie(token);
            HttpResponse::Ok().cookie(cookie).json(ApiResponse::success(
                messages.get_auth_message("login.success", "Login successful"),
                user,
            ))
        }
        Err(err) => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            err.to_string(),
            ErrorDetails { details: None },
        )),
    }
}

pub async fn logout_user_handler(req: HttpRequest) -> HttpResponse {
    use actix_web::cookie::{Cookie, SameSite, time::Duration};

    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let expired = Cookie::build(&*COOKIE_NAME, "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::new(0, 0))
        .finish();

    HttpResponse::Ok()
        .cookie(expired)
        .json(ApiResponse::success(
            messages.get_auth_message("logout.success", "Logged out successfully."),
            None::<()>,
        ))
}
