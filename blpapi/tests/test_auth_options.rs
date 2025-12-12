use blpapi::auth_options::{
    AuthApplicationBuilder, AuthOptionsBuilder, AuthTokenBuilder, AuthUserBuilder, ManualOptions,
};
use blpapi::core::{
    BLPAPI_DEFAULT_DIRECTORY_SERVICE, BLPAPI_DEFAULT_HOST, BLPAPI_DEFAULT_SESSION_NAME,
};
use blpapi::session_options::Authentication;

#[test]
pub fn test_auth_options_user_builder() {
    let builder = AuthUserBuilder::default();
    let options = builder.manual_options.clone();
    let options_two = builder.manual_options.clone();
    assert_eq!(builder.authentication_mode, Some(Authentication::OsLogon));
    assert_eq!(
        builder.active_directory,
        Some(BLPAPI_DEFAULT_DIRECTORY_SERVICE.into())
    );
    assert_eq!(options.unwrap().user_id, BLPAPI_DEFAULT_SESSION_NAME);
    assert_eq!(options_two.unwrap().ip_address, BLPAPI_DEFAULT_HOST);
}

#[test]
pub fn test_auth_options_user_logonname() {
    let logon_name = Authentication::OsLogon;
    let builder = AuthUserBuilder::default();
    let builder = builder.set_logon_name(logon_name);
    let auth = builder.build();
    drop(auth);
}

#[test]
pub fn test_auth_options_user_active_directory() {
    let act_dir = BLPAPI_DEFAULT_DIRECTORY_SERVICE;
    let builder = AuthUserBuilder::default();
    let builder = builder.set_active_directory(act_dir);
    let auth = builder.build();
    drop(auth);
}

#[test]
pub fn test_auth_options_user_manual_options() {
    let manual_options = ManualOptions::default();
    let builder = AuthUserBuilder::default();
    let builder = builder.set_manual_options(manual_options);
    let auth = builder.build();
    drop(auth);
}

#[test]
pub fn test_auth_options_user_duplicate() {
    let builder = AuthUserBuilder::default();
    let auth_user = builder.build();
    let _new_auth_user = auth_user.clone();
    drop(_new_auth_user);
}

#[test]
pub fn test_auth_options_user_destroy() {
    let act_dir = BLPAPI_DEFAULT_DIRECTORY_SERVICE;
    let builder = AuthUserBuilder::new();
    let builder = builder.set_active_directory(act_dir);
    let auth = builder.build();
    drop(auth);
}

#[test]
pub fn test_auth_options_token_builder() {
    let builder = AuthTokenBuilder::default();
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_token_builder_new_auth() {
    let new_auth = String::from("NewAuth");
    let builder = AuthTokenBuilder::default();
    let builder = builder.set_auth_token(new_auth);
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_token_duplicate() {
    let builder = AuthTokenBuilder::default();
    let auth_token = builder.build();
    let _new_auth_token = auth_token.clone();
    drop(_new_auth_token);
    drop(auth_token);
}

#[test]
pub fn test_auth_options_token_destroy() {
    let builder = AuthTokenBuilder::default();
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_app_builder() {
    let builder = AuthApplicationBuilder::default();
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_app_builder_new_app() {
    let new_app = String::from("NewApp");
    let builder = AuthApplicationBuilder::default();
    let builder = builder.set_auth_app(new_app);
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_app_duplicate() {
    let builder = AuthApplicationBuilder::default();
    let auth_token = builder.build();
    let _new_auth_app = auth_token.clone();
    assert_eq!(auth_token.auth_application, _new_auth_app.auth_application);
    drop(_new_auth_app);
    drop(auth_token);
}

#[test]
pub fn test_auth_options_app_destroy() {
    let builder = AuthApplicationBuilder::default();
    let auth_token = builder.build();
    drop(auth_token);
}

#[test]
pub fn test_auth_options_default() {
    let builder = AuthOptionsBuilder::default();
    let auth_options = builder.build();
    drop(auth_options);
}

#[test]
pub fn test_auth_options_user_mode() {
    let user = AuthUserBuilder::default();
    let user = user.build();

    let builder = AuthOptionsBuilder::default();
    let builder = builder.set_auth_user(user);

    let auth_options = builder.build();
    drop(auth_options);
}

#[test]
pub fn test_auth_options_app_mode() {
    let app = AuthApplicationBuilder::default();
    let app = app.build();

    let builder = AuthOptionsBuilder::default();
    let builder = builder.set_auth_application(app);

    let auth_options = builder.build();
    drop(auth_options);
}

#[test]
pub fn test_auth_options_user_app_mode() {
    let app = AuthApplicationBuilder::default();
    let app = app.build();

    let user = AuthUserBuilder::default();
    let user = user.build();

    let builder = AuthOptionsBuilder::default();
    let builder = builder.set_auth_application(app).set_auth_user(user);
    let auth_options = builder.build();
    drop(auth_options);
}

#[test]
pub fn test_auth_options_token_mode() {
    let builder = AuthTokenBuilder::default();
    let auth_token = builder.build();

    let builder = AuthOptionsBuilder::default();
    let builder = builder.set_auth_token(auth_token);
    let auth_options = builder.build();
    drop(auth_options);
}

#[test]
pub fn test_auth_options_clone() {
    let builder = AuthOptionsBuilder::default();
    let auth_options = builder.build();
    let clone_options = auth_options.clone();
    drop(clone_options);
    drop(auth_options);
}

#[test]
pub fn test_auth_options_destroy() {
    let builder = AuthOptionsBuilder::default();
    let auth_options = builder.build();
    drop(auth_options);
}
