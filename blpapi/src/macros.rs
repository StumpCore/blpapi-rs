#[macro_export]
macro_rules! overrides {
    ($($name:ident = $val:expr),* $(,)?) => {
        &vec![
            $(
                $crate::overrides::Override::new(stringify!($name), $val),
            )*
        ]
    };
}

#[macro_export]
macro_rules! table_overrides {
    ($($name:ident = $row:expr),* $(,)?) => {
        &vec![
            $(
                $crate::overrides::TableOverride::new(stringify!($name), $row),
            )*
        ]
    };
}
