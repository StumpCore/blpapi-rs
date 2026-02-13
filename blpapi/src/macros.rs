#[macro_export]
macro_rules! overrides {
    ($($name:ident = $val:expr),* $(,)?) => {
        &vec![
            $(
                $crate::overrides::Override::new(stringify!($name), stringify!($val)),
            )*
        ]
    };
}

#[macro_export]
macro_rules! table_overrides {
    ($($name:ident = $row:expr),* $(,)?) => {
        &vec![
            $(
                $crate::overrides::TableOverride::new(stringify!($name), stringify!($row)),
            )*
        ]
    };
}

#[macro_export]
macro_rules! options{
    ($($name:ident = $val:expr),* $(,)?) => {
        vec![
            $(
                $crate::overrides::SubscribeOption::new(stringify!($name), stringify!($val)),
            )*
        ]
    };
}
