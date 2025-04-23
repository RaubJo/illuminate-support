use crate::Fluent;

#[macro_export]
macro_rules! fluent {
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        let mut temp = Fluent::new();
        $(
            temp.set(stringify!($key), $value);
        )*
        temp
    }};
}
