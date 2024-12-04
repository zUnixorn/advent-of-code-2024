pub mod template;

#[macro_export]
macro_rules! parse {
    ($item:expr, $datatype:ident) => {
        $item.parse::<$datatype>().unwrap();
    };
}
