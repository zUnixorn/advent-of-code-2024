use std::str::FromStr;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::IResult;

pub mod template;

#[macro_export]
macro_rules! parse {
    ($item:expr, $datatype:ident) => {
        $item.parse::<$datatype>().unwrap();
    };
}

pub fn number<T: FromStr>(input : &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}
