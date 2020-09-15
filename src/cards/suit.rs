use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::*;

/// Suit struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Suit {
    pub value: isize,
    pub raw: String,
}

impl Suit {
    pub fn new<S: std::clone::Clone>(name: S) -> Suit
    where
        S: Into<String>,
    {
        let n = name.into();
        let value = get_value_isize(n.as_str());
        Suit::new_with_value(n, value)
    }

    pub fn new_with_value<S: std::clone::Clone>(name: S, value: isize) -> Suit
    where
        S: Into<String>,
    {
        Suit {
            value,
            raw: name.into(),
        }
    }

    pub fn get_short(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-letter", self.raw);
        get_value_by_key(key.as_str(), lid)
    }

    pub fn get_default_long(&self) -> String {
        self.get_long(&US_ENGLISH)
    }

    pub fn get_long(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-name", self.raw);
        get_value_by_key(key.as_str(), lid)
    }

    pub fn get_symbol(&self) -> String {
        let key = format!("{}-symbol", self.raw);
        get_value_by_key(key.as_str(), &US_ENGLISH)
    }

    fn bottom_up_value(_len: usize, i: usize) -> isize {
        (i + 1) as isize
    }

    fn top_down_value(len: usize, i: usize) -> isize {
        (len - i) as isize
    }

    fn from_array_gen(s: &[&str], f: impl Fn(usize, usize) -> isize) -> Vec<Suit> {
        let mut v: Vec<Suit> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let value = f(s.len(), i);
            v.push(Suit::new_with_value(elem, value));
        }
        v
    }

    pub fn from_array(s: &[&str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::top_down_value)
    }

    pub fn from_array_bottom_up(s: &[&str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::bottom_up_value)
    }

    pub fn generate_french_suits() -> Vec<Suit> {
        Suit::from_array(&["spades", "hearts", "diamonds", "clubs"])
    }

    pub fn generate_arcana_suits() -> Vec<Suit> {
        Suit::from_array(&["major-arcana", "wands", "cups", "swords", "pentacles"])
    }

    pub fn generate_skat_suits() -> Vec<Suit> {
        Suit::from_array(&["eichel", "laub", "herz", "schellen"])
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl Valuable for Suit {
    fn revise_value(&mut self, new_value: isize) {
        self.value = new_value
    }

    fn get_value(&self) -> isize {
        self.value
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_tests {
    use super::*;
    use crate::fluent::GERMAN;

    #[test]
    fn display() {
        assert_eq!("Suit: ♥", format!("Suit: {}", Suit::new("hearts")));
    }

    #[test]
    fn new() {
        let expected = Suit {
            value: 4,
            raw: "spades".to_string(),
        };

        assert_eq!(expected, Suit::new("spades"));
    }

    #[test]
    fn new_with_value() {
        let expected = Suit {
            value: 4,
            raw: "spades".to_string(),
        };

        assert_eq!(expected, Suit::new_with_value("spades", 4));
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Suit::new_with_value("spades", 3),
            Suit::new_with_value("spades", 4)
        );
        assert_ne!(
            Suit::new_with_value("hearts", 4),
            Suit::new_with_value("spades", 4)
        );
    }

    #[test]
    fn get_short() {
        let clubs = Suit::new("clubs");

        assert_eq!("C".to_string(), clubs.get_short(&US_ENGLISH));
        assert_eq!("K".to_string(), clubs.get_short(&GERMAN));
    }

    #[test]
    fn get_symbol() {
        let clubs = Suit::new("clubs");

        assert_eq!("♣".to_string(), clubs.get_symbol());
    }

    #[test]
    fn get_long() {
        let clubs = Suit::new("clubs");

        assert_eq!("Clubs".to_string(), clubs.get_long(&US_ENGLISH));
        assert_eq!("Klee".to_string(), clubs.get_long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Suit::new("clubs").to_string(), "♣".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Suit::new("clubs");

        assert_eq!(clubs.get_short(&GERMAN), "K".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("clubs", 2));
        expected.push(Suit::new_with_value("spades", 1));

        assert_eq!(expected, Suit::from_array(&["clubs", "spades"]));
    }

    #[test]
    fn to_vec_bottom_up() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("clubs", 1));
        expected.push(Suit::new_with_value("spades", 2));

        assert_eq!(expected, Suit::from_array_bottom_up(&["clubs", "spades"]));
    }

    #[test]
    fn generate_french_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("spades", 4));
        expected.push(Suit::new_with_value("hearts", 3));
        expected.push(Suit::new_with_value("diamonds", 2));
        expected.push(Suit::new_with_value("clubs", 1));

        assert_eq!(expected, Suit::generate_french_suits());
    }

    #[test]
    fn generate_arcana_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("major-arcana", 5));
        expected.push(Suit::new_with_value("wands", 4));
        expected.push(Suit::new_with_value("cups", 3));
        expected.push(Suit::new_with_value("swords", 2));
        expected.push(Suit::new_with_value("pentacles", 1));

        assert_eq!(expected, Suit::generate_arcana_suits());
    }

    #[test]
    fn revise_value() {
        let mut wands = Suit::new("wands");
        assert_eq!(4, wands.get_value());

        wands.revise_value(3);

        assert_eq!(3, wands.get_value());
    }
}
