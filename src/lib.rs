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

pub fn number_u32(input : &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}
