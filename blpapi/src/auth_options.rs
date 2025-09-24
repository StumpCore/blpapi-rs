use crate::core::{BLPAPI_AUTHENTICATION_APPNAME_AND_KEY, BLPAPI_AUTHENTICATION_OS_LOGON, BLPAPI_DEFAULT_DIRECTORY_SERVICE, BLPAPI_DEFAULT_HOST, BLPAPI_DEFAULT_SESSION_NAME};
use crate::session_options::Authentication;
use blpapi_sys::{blpapi_AuthApplication_create, blpapi_AuthApplication_destroy, blpapi_AuthApplication_duplicate, blpapi_AuthApplication_t, blpapi_AuthOptions, blpapi_AuthOptions_create_default, blpapi_AuthOptions_create_forAppMode, blpapi_AuthOptions_create_forToken, blpapi_AuthOptions_create_forUserAndAppMode, blpapi_AuthOptions_create_forUserMode, blpapi_AuthOptions_destroy, blpapi_AuthOptions_duplicate, blpapi_AuthOptions_t, blpapi_AuthToken_create, blpapi_AuthToken_destroy, blpapi_AuthToken_duplicate, blpapi_AuthToken_t, blpapi_AuthUser_createWithActiveDirectoryProperty, blpapi_AuthUser_createWithLogonName, blpapi_AuthUser_createWithManualOptions, blpapi_AuthUser_destroy, blpapi_AuthUser_duplicate, blpapi_AuthUser_t};
use std::ffi::CString;

/// Manual Options for the AuthUser
#[derive(Debug, Clone, PartialEq)]
pub struct ManualOptions {
    pub user_id: String,
    pub ip_address: String,
}

/// Implementing default details for the manual options
impl Default for ManualOptions {
    fn default() -> Self {
        Self {
            user_id: BLPAPI_DEFAULT_SESSION_NAME.into(),
            ip_address: BLPAPI_DEFAULT_HOST.into(),
        }
    }
}

/// AuthUser Builder
#[derive(Debug, Clone, PartialEq)]
pub struct AuthUserBuilder {
    pub(crate) auth_user: *mut blpapi_AuthUser_t,
    pub active_directory: Option<String>,
    pub authentication_mode: Option<Authentication>,
    pub manual_options: Option<ManualOptions>,
}

impl Default for AuthUserBuilder {
    fn default() -> AuthUserBuilder {
        let ptr: *mut blpapi_AuthUser_t = std::ptr::null_mut();
        Self {
            auth_user: ptr,
            active_directory: Some(BLPAPI_DEFAULT_DIRECTORY_SERVICE.into()),
            authentication_mode: Some(Authentication::OsLogon),
            manual_options: Some(ManualOptions {
                user_id: BLPAPI_DEFAULT_SESSION_NAME.into(),
                ip_address: BLPAPI_DEFAULT_HOST.into(),
            }),
        }
    }
}

impl AuthUserBuilder {
    pub fn new() -> Self {
        let ptr: *mut blpapi_AuthUser_t = std::ptr::null_mut();
        Self {
            auth_user: ptr,
            authentication_mode: None,
            active_directory: None,
            manual_options: None,
        }
    }

    /// Setting the active directory property
    pub fn set_active_directory<T: Into<String>>(mut self, new_directory: T) -> Self {
        let new_directory = new_directory.into();
        self.active_directory = Some(new_directory);
        self
    }

    /// Setting the logon Name
    pub fn set_logon_name(mut self, new_mode: Authentication) -> Self {
        self.authentication_mode = Some(new_mode);
        self
    }

    /// setting the manual options
    pub fn set_manual_options(mut self, new_options: ManualOptions) -> Self {
        self.manual_options = Some(new_options);
        self
    }

    /// build the AuthUser
    /// The create function creates based on the following order
    /// Authentication > Active Directory > Manual Options
    /// In case one of the lesser is required, the fields need to be set to 'None' by creating
    /// a new instance and setting only the desired property
    pub fn build(self) -> AuthUser {
        let mut ptr: *mut blpapi_AuthUser_t = self.auth_user;
        if self.authentication_mode.is_some() {
            unsafe {
                let i = blpapi_AuthUser_createWithLogonName(
                    &mut ptr,
                );
                if i != 0 {
                    panic!("Failed to generate logon name");
                };
            };
        } else if self.active_directory.is_some() {
            let property = self.active_directory.unwrap_or(BLPAPI_DEFAULT_DIRECTORY_SERVICE.into());
            let property = CString::new(property).expect("Failed to generate directory property");

            unsafe {
                let i = blpapi_AuthUser_createWithActiveDirectoryProperty(
                    &mut ptr,
                    property.as_ptr(),
                );
                if i != 0 {
                    panic!("Failed to generate active directory property");
                };
            };
        } else if self.manual_options.is_some() {
            let property = self.manual_options.expect("Expected Manual Options");
            let id = property.user_id;
            let id_c = CString::new(id).expect("Failed to generate manual id");
            let ip_address = property.ip_address;
            let ip_address_c = CString::new(ip_address).expect("Failed to generate manual ip address");

            unsafe {
                let i = blpapi_AuthUser_createWithManualOptions(
                    &mut ptr,
                    id_c.as_ptr(),
                    ip_address_c.as_ptr(),
                );
                if i != 0 {
                    panic!("Failed to generate manual options");
                };
            };
        }

        AuthUser {
            ptr,
        }
    }
}

/// AuthUser struct
#[derive(Debug, PartialEq)]
pub struct AuthUser {
    pub(crate) ptr: *mut blpapi_AuthUser_t,
}

impl Clone for AuthUser {
    fn clone(&self) -> Self {
        let mut new_id: *mut blpapi_AuthUser_t = std::ptr::null_mut();
        let ptr: *const blpapi_AuthUser_t = self.ptr;
        unsafe {
            let i = blpapi_AuthUser_duplicate(
                &mut new_id,
                ptr,
            );
            if i != 0 {
                panic!("Failed to duplicate auth user");
            }
        };
        AuthUser {
            ptr: new_id,
        }
    }
}

/// Implement the Drop trait
impl Drop for AuthUser {
    fn drop(&mut self) {
        unsafe {
            blpapi_AuthUser_destroy(self.ptr)
        }
    }
}


/// Auth Token Builder struct
#[derive(Debug, Clone, PartialEq)]
pub struct AuthTokenBuilder {
    pub(crate) ptr: *mut blpapi_AuthToken_t,
    pub auth_token: String,
}

impl Default for AuthTokenBuilder {
    fn default() -> AuthTokenBuilder {
        let ptr: *mut blpapi_AuthToken_t = std::ptr::null_mut();
        Self {
            ptr: ptr,
            auth_token: BLPAPI_AUTHENTICATION_OS_LOGON.into(),
        }
    }
}

impl AuthTokenBuilder {
    pub fn set_auth_token(mut self, auth_token: String) -> Self {
        self.auth_token = auth_token;
        self
    }

    pub fn build(self) -> AuthToken {
        let mut ptr: *mut blpapi_AuthToken_t = self.ptr;
        let auth_token = self.auth_token;
        let auth_token_c = CString::new(auth_token).expect("Failed to generate auth token");
        unsafe {
            let i = blpapi_AuthToken_create(
                &mut ptr,
                auth_token_c.as_ptr(),
            );
            if i != 0 {
                panic!("Failed to create auth token");
            };
        }
        AuthToken {
            ptr
        }
    }
}

/// Auth Token struct
#[derive(Debug, PartialEq)]
pub struct AuthToken {
    pub(crate) ptr: *mut blpapi_AuthToken_t,
}

impl Clone for AuthToken {
    fn clone(&self) -> Self {
        let mut new_token: *mut blpapi_AuthToken_t = std::ptr::null_mut();
        let ptr: *const blpapi_AuthToken_t = self.ptr;
        unsafe {
            let i = blpapi_AuthToken_duplicate(
                &mut new_token,
                ptr,
            );
            if i != 0 {
                panic!("Failed to duplicate auth token");
            }
        };
        AuthToken {
            ptr: new_token,
        }
    }
}

/// Implement the Drop trait
impl Drop for AuthToken {
    fn drop(&mut self) {
        unsafe {
            blpapi_AuthToken_destroy(self.ptr)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthApplicationBuilder {
    pub(crate) ptr: *mut blpapi_AuthApplication_t,
    pub auth_app: String,
}

impl Default for AuthApplicationBuilder {
    fn default() -> AuthApplicationBuilder {
        let ptr: *mut blpapi_AuthApplication_t = std::ptr::null_mut();
        Self {
            ptr: ptr,
            auth_app: BLPAPI_AUTHENTICATION_APPNAME_AND_KEY.into(),
        }
    }
}

impl AuthApplicationBuilder {
    pub fn set_auth_app(mut self, app: String) -> Self {
        self.auth_app = app;
        self
    }

    pub fn build(self) -> AuthApplication {
        let mut ptr: *mut blpapi_AuthApplication_t = self.ptr;
        let auth_app = self.auth_app.clone();
        let auth_app_name = self.auth_app;
        let auth_app_c = CString::new(auth_app).expect("Failed to generate auth application");
        unsafe {
            let i = blpapi_AuthApplication_create(
                &mut ptr,
                auth_app_c.as_ptr(),
            );
            if i != 0 {
                panic!("Failed to create auth application");
            };
        }
        AuthApplication {
            ptr,
            auth_application: auth_app_name,
        }
    }
}


/// Auth Application struct
#[derive(Debug, PartialEq)]
pub struct AuthApplication {
    pub(crate) ptr: *mut blpapi_AuthApplication_t,
    pub auth_application: String,
}

impl Clone for AuthApplication {
    fn clone(&self) -> Self {
        let mut new_app: *mut blpapi_AuthApplication_t = std::ptr::null_mut();
        let ptr: *const blpapi_AuthApplication_t = self.ptr;
        unsafe {
            let i = blpapi_AuthApplication_duplicate(
                &mut new_app,
                ptr,
            );
            if i != 0 {
                panic!("Failed to duplicate auth app");
            }
        };
        AuthApplication {
            ptr: new_app,
            auth_application: self.auth_application.clone(),
        }
    }
}

/// Implement the Drop trait
impl Drop for AuthApplication {
    fn drop(&mut self) {
        unsafe {
            blpapi_AuthApplication_destroy(self.ptr)
        }
    }
}


/// AuthOptionsBuilder
#[derive(Debug, PartialEq)]
pub struct AuthOptionsBuilder {
    pub(crate) ptr: *mut blpapi_AuthOptions,
    pub auth_user: Option<AuthUser>,
    pub auth_application: Option<AuthApplication>,
    pub auth_token: Option<AuthToken>,
}

/// AuthOptionsBuilder Default Trait
impl Default for AuthOptionsBuilder {
    fn default() -> AuthOptionsBuilder {
        let ptr: *mut blpapi_AuthOptions_t = std::ptr::null_mut();
        Self {
            ptr: ptr,
            auth_user: None,
            auth_application: None,
            auth_token: None,
        }
    }
}

impl AuthOptionsBuilder {
    /// Setting Authuser
    pub fn set_auth_user(mut self, user: AuthUser) -> Self {
        self.auth_user = Some(user);
        self
    }

    /// Setting AuthApplication
    pub fn set_auth_application(mut self, application: AuthApplication) -> Self {
        self.auth_application = Some(application);
        self
    }

    /// Setting AuthToken
    pub fn set_auth_token(mut self, token: AuthToken) -> Self {
        self.auth_token = Some(token);
        self
    }

    ///Building the AuthOptions
    pub fn build(self) -> AuthOptions {
        let default = self.auth_token.is_none() && self.auth_application.is_none() && self.auth_user.is_none();
        let user_mode = self.auth_user.is_some() && self.auth_application.is_none() && self.auth_token.is_none();
        let token_mode = self.auth_token.is_some() && self.auth_application.is_none() && self.auth_user.is_none();
        let app_mode = self.auth_application.is_some() && self.auth_token.is_none() && self.auth_user.is_none();
        let user_app_mode = self.auth_application.is_some() && self.auth_token.is_none() && self.auth_user.is_some();

        let mut ptr: *mut blpapi_AuthOptions_t = self.ptr;
        // Create default
        if default {
            unsafe {
                let i = blpapi_AuthOptions_create_default(
                    &mut ptr,
                );
                if i != 0 {
                    panic!("Failed to create auth options (default)");
                };
            }
        } else if user_mode {
            let user = self.auth_user.expect("Expected AuthUser, set first");
            let auth_ptr = user.ptr;

            unsafe {
                let i = blpapi_AuthOptions_create_forUserMode(
                    &mut ptr,
                    auth_ptr,
                );
                if i != 0 {
                    panic!("Failed to create auth options (Usermode)");
                };
            }
        } else if app_mode {
            let app = self.auth_application.expect("Expected AuthApplication, set first");
            let auth_ptr = app.ptr;

            unsafe {
                let i = blpapi_AuthOptions_create_forAppMode(
                    &mut ptr,
                    auth_ptr,
                );
                if i != 0 {
                    panic!("Failed to create auth options (AppMode)");
                };
            }
        } else if user_app_mode {
            let app = self.auth_application.expect("Expected AuthApplication, set first");
            let user = self.auth_user.expect("Expected AuthUser, set first");
            let app_ptr = app.ptr;
            let user_ptr = user.ptr;

            unsafe {
                let i = blpapi_AuthOptions_create_forUserAndAppMode(
                    &mut ptr,
                    user_ptr,
                    app_ptr,
                );
                if i != 0 {
                    panic!("Failed to create auth options (UserAppMode)");
                };
            }
        } else if token_mode {
            let token = self.auth_token.expect("Expected AuthToken, set first");
            let token_ptr = token.ptr;

            unsafe {
                let i = blpapi_AuthOptions_create_forToken(
                    &mut ptr,
                    token_ptr,
                );
                if i != 0 {
                    panic!("Failed to create auth options (TokenMode)");
                };
            }
        }

        AuthOptions {
            ptr,
        }
    }
}

/// AuthOptions struct
#[derive(Debug, PartialEq)]
pub struct AuthOptions {
    pub(crate) ptr: *mut blpapi_AuthOptions_t,

}

impl Clone for AuthOptions {
    fn clone(&self) -> Self {
        let mut new_opt: *mut blpapi_AuthOptions_t = std::ptr::null_mut();
        let ptr: *const blpapi_AuthOptions_t = self.ptr;
        unsafe {
            let i = blpapi_AuthOptions_duplicate(
                &mut new_opt,
                ptr,
            );
            if i != 0 {
                panic!("Failed to duplicate auth options");
            }
        };
        AuthOptions {
            ptr: new_opt,
        }
    }
}

/// Implement the Drop trait
impl Drop for AuthOptions {
    fn drop(&mut self) {
        unsafe {
            blpapi_AuthOptions_destroy(self.ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auth_options::{AuthApplicationBuilder, AuthOptionsBuilder, AuthTokenBuilder, AuthUserBuilder, ManualOptions};
    use crate::core::{BLPAPI_DEFAULT_DIRECTORY_SERVICE, BLPAPI_DEFAULT_HOST, BLPAPI_DEFAULT_SESSION_NAME};
    use crate::session_options::Authentication;

    #[test]
    pub fn test_auth_user_builder() {
        let builder = AuthUserBuilder::default();
        let options = builder.manual_options.clone();
        let options_two = builder.manual_options.clone();
        assert_eq!(builder.authentication_mode, Some(Authentication::OsLogon));
        assert_eq!(builder.active_directory, Some(BLPAPI_DEFAULT_DIRECTORY_SERVICE.into()));
        assert_eq!(options.unwrap().user_id, BLPAPI_DEFAULT_SESSION_NAME);
        assert_eq!(options_two.unwrap().ip_address, BLPAPI_DEFAULT_HOST);
    }

    #[test]
    pub fn test_auth_user_logonname() {
        let logon_name = Authentication::OsLogon;
        let builder = AuthUserBuilder::default();
        let builder = builder.set_logon_name(logon_name);
        let _auth = builder.build();
    }

    #[test]
    pub fn test_auth_user_active_directory() {
        let act_dir = BLPAPI_DEFAULT_DIRECTORY_SERVICE;
        let builder = AuthUserBuilder::default();
        let builder = builder.set_active_directory(act_dir);
        let _auth = builder.build();
    }

    #[test]
    pub fn test_auth_user_manual_options() {
        let manual_options = ManualOptions::default();
        let builder = AuthUserBuilder::default();
        let builder = builder.set_manual_options(manual_options);
        let _auth = builder.build();
    }

    #[test]
    pub fn test_auth_user_duplicate() {
        let builder = AuthUserBuilder::default();
        let auth_user = builder.build();
        let _new_auth_user = auth_user.clone();
    }

    #[test]
    pub fn test_auth_user_destroy() {
        let act_dir = BLPAPI_DEFAULT_DIRECTORY_SERVICE;
        let builder = AuthUserBuilder::new();
        let builder = builder.set_active_directory(act_dir);
        let auth = builder.build();
        drop(auth);
    }

    #[test]
    pub fn test_auth_token_builder() {
        let builder = AuthTokenBuilder::default();
        println!("{:?}", builder);
        let auth_token = builder.build();
        println!("{:?}", auth_token);
    }

    #[test]
    pub fn test_auth_token_builder_new_auth() {
        let new_auth = String::from("NewAuth");
        let builder = AuthTokenBuilder::default();
        println!("{:?}", builder);
        let builder = builder.set_auth_token(new_auth);
        println!("{:?}", builder);
        let auth_token = builder.build();
        println!("{:?}", auth_token);
    }

    #[test]
    pub fn test_auth_token_duplicate() {
        let builder = AuthTokenBuilder::default();
        let auth_token = builder.build();
        let _new_auth_token = auth_token.clone();
    }

    #[test]
    pub fn test_auth_token_destroy() {
        let builder = AuthTokenBuilder::default();
        let auth_token = builder.build();
        drop(auth_token);
    }

    #[test]
    pub fn test_auth_app_builder() {
        let builder = AuthApplicationBuilder::default();
        println!("{:?}", builder);
        let auth_token = builder.build();
        println!("{:?}", auth_token);
    }

    #[test]
    pub fn test_auth_app_builder_new_app() {
        let new_app = String::from("NewApp");
        let builder = AuthApplicationBuilder::default();
        println!("{:?}", builder);
        let builder = builder.set_auth_app(new_app);
        println!("{:?}", builder);
        let auth_token = builder.build();
        println!("{:?}", auth_token);
    }

    #[test]
    pub fn test_auth_app_duplicate() {
        let builder = AuthApplicationBuilder::default();
        let auth_token = builder.build();
        let _new_auth_app = auth_token.clone();
        assert_eq!(auth_token.auth_application, _new_auth_app.auth_application);
    }

    #[test]
    pub fn test_auth_app_destroy() {
        let builder = AuthApplicationBuilder::default();
        let auth_token = builder.build();
        drop(auth_token);
    }

    #[test]
    pub fn test_auth_options_default() {
        let builder = AuthOptionsBuilder::default();
        println!("{:?}", builder);
        let auth_options = builder.build();
        println!("{:?}", auth_options);
    }

    #[test]
    pub fn test_auth_options_user_mode() {
        let user = AuthUserBuilder::default();
        let user = user.build();
        println!("{:?}", &user);

        let builder = AuthOptionsBuilder::default();
        let builder = builder.set_auth_user(user);
        println!("{:?}", builder);

        let auth_options = builder.build();
        println!("{:?}", auth_options);
    }

    #[test]
    pub fn test_auth_options_app_mode() {
        let app = AuthApplicationBuilder::default();
        let app = app.build();
        println!("{:?}", &app);

        let builder = AuthOptionsBuilder::default();
        let builder = builder.set_auth_application(app);
        println!("{:?}", builder);

        let auth_options = builder.build();
        println!("{:?}", auth_options);
    }

    #[test]
    pub fn test_auth_options_user_app_mode() {
        let app = AuthApplicationBuilder::default();
        let app = app.build();
        println!("{:?}", &app);

        let user = AuthUserBuilder::default();
        let user = user.build();
        println!("{:?}", &user);

        let builder = AuthOptionsBuilder::default();
        let builder = builder.set_auth_application(app).set_auth_user(user);
        println!("{:?}", builder);

        let auth_options = builder.build();
        println!("{:?}", auth_options);
    }

    #[test]
    pub fn test_auth_options_token_mode() {
        let builder = AuthTokenBuilder::default();
        let auth_token = builder.build();
        println!("{:?}", &auth_token);

        let builder = AuthOptionsBuilder::default();
        let builder = builder.set_auth_token(auth_token);
        println!("{:?}", builder);

        let auth_options = builder.build();
        println!("{:?}", auth_options);
    }

    #[test]
    pub fn test_auth_options_clone() {
        let builder = AuthOptionsBuilder::default();
        println!("{:?}", builder);
        let auth_options = builder.build();
        println!("{:?}", auth_options);
        let clone_options = auth_options.clone();
        println!("{:?}", clone_options);
    }

    #[test]
    pub fn test_auth_options_destroy() {
        let builder = AuthOptionsBuilder::default();
        println!("{:?}", builder);
        let auth_options = builder.build();
        println!("{:?}", auth_options);
        drop(auth_options);
    }
}