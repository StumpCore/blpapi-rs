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
