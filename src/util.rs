use std::fmt::Display;

pub trait Formatf64
where Self: Display{
    fn to_string_two_bits(&self) -> String {
        format!("{:.2}", self)
    }
}