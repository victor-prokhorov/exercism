#[macro_export]
macro_rules! hashmap {
    ($($($i:expr => $j:expr)+$(,)?)*) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $($(hashmap.insert($i, $j);)+)*
        hashmap
    }};
}
