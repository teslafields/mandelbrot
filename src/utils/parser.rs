use std::str::FromStr;
use num::Complex;
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                    _ => None
            }
        }
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parser::parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parser::parse_pair::<i32>("10,", ','), None);
}
#[test]
fn test_parse_complex() {
    assert_eq!(parser::parse_complex("1.25,-0.065"), Some(Complex{ re: 1.25, im: -0.065 }));
    assert_eq!(parser::parse_complex(",-0.065"), None);
}
