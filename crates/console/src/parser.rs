#![allow(unused)]
use std::str::FromStr;

/// Parse a single type from string
pub fn auto_parse<T: FromStr>(input: String) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    T::from_str(&input).map_err(|e| e.to_string())
}

/// Macro to generate tuple parsers
macro_rules! tuple_parser {
    ($($ty:ty),+) => {
        |input: String| -> Result<($($ty,)+), String> {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let mut index = 0;
            (
                $(
                    {
                        if index >= parts.len() {
                            return Err(format!("Expected more arguments"));
                        }
                        let val = <$ty>::from_str(parts[index])
                            .map_err(|_| format!("Failed to parse argument {}", index))?;
                        index += 1;
                        val
                    }
                ),+
            )
            .into()
        }
    };
}
