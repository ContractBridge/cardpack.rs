use crate::fluent::{ToLocaleString, US_ENGLISH};
use std::fmt;

/// Karten Anzug Name (Card Suit Letter) - Single field struct representing the letter of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SuitLetter(String);

impl SuitLetter {
    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    pub fn new<S>(name: S) -> SuitLetter
    where
        S: Into<String>,
    {
        SuitLetter(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.get_raw_name()
    }
}

impl ToLocaleString for SuitLetter {
    fn get_fluent_key(&self) -> String {
        self.0.to_owned() + &*"-letter".to_owned()
    }

    fn get_raw_name(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SuitLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_letter_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "AnzugBuchstabe: H",
            format!("AnzugBuchstabe: {}", SuitLetter::new("hearts"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(SuitLetter::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(SuitLetter::new("diamonds").to_string(), "D".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(SuitLetter("from".to_string()), SuitLetter::new(from_string));
        assert_eq!(SuitLetter("from".to_string()), SuitLetter::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = SuitLetter::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "K".to_string());
    }
}
