use std::env;
use std::sync::LazyLock;

macro_rules! lazy_env_var {
    ($name:ident) => {
        pub static $name: LazyLock<String> = LazyLock::new(|| {
            let var_name = stringify!($name);
            env::var(var_name).expect(&format!("{} must be set", var_name))
        });
    };
}

lazy_env_var!(CLIENT_ID);
lazy_env_var!(CLIENT_SECRET);
lazy_env_var!(REDIRECT_URI);
lazy_env_var!(JWT_SECRET_KEY);
lazy_env_var!(COOKIE_NAME);
lazy_env_var!(MONGODB_URI);
lazy_env_var!(DB_NAME);
lazy_env_var!(USER_COL_NAME);
lazy_env_var!(CLASS_COL_NAME);
lazy_env_var!(ATTENDANCE_COL_NAME);
lazy_env_var!(ORGANIZATIONS_COL_NAME);
