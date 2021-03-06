use std::fmt;

use crate::fluent_name::FluentName;
use crate::Named;
use crate::US_ENGLISH;

// Constants representing the internal identifier for a Rank inside the Fluent template files.
// French Deck Ranks:
pub const ACE: &str = "ace";
pub const KING: &str = "king";
pub const QUEEN: &str = "queen";
pub const JACK: &str = "jack";
pub const TEN: &str = "ten";
pub const NINE: &str = "nine";
pub const EIGHT: &str = "eight";
pub const SEVEN: &str = "seven";
pub const SIX: &str = "six";
pub const FIVE: &str = "five";
pub const FOUR: &str = "four";
pub const THREE: &str = "three";
pub const TWO: &str = "two";
// Spades etc Ranks:
pub const BIG_JOKER: &str = "big-joker";
pub const LITTLE_JOKER: &str = "little-joker";
// Skat Deck Ranks:
pub const DAUS: &str = "daus";
pub const OBER: &str = "ober";
pub const UNTER: &str = "unter";
// Tarot Deck Ranks:
pub const FOOL: &str = "fool";
pub const MAGICIAN: &str = "magician";
pub const PRIESTESS: &str = "priestess";
pub const EMPRESS: &str = "empress";
pub const EMPEROR: &str = "emperor";
pub const HIEROPHANT: &str = "hierophant";
pub const LOVERS: &str = "lovers";
pub const CHARIOT: &str = "chariot";
pub const STRENGTH: &str = "strength";
pub const HERMIT: &str = "hermit";
pub const FORTUNE: &str = "fortune";
pub const JUSTICE: &str = "justice";
pub const HANGED: &str = "hanged";
pub const DEATH: &str = "death";
pub const TEMPERANCE: &str = "temperance";
pub const DEVIL: &str = "devil";
pub const TOWER: &str = "tower";
pub const STAR: &str = "star";
pub const MOON: &str = "moon";
pub const SUN: &str = "sun";
pub const JUDGEMENT: &str = "judgement";
pub const WORLD: &str = "world";
pub const KNIGHT: &str = "knight";
pub const PAGE: &str = "page";

/// Rank Struct for a Card. Examples of standard Card Ranks would include: Ace, Ten, and Deuce
/// Joker, Death (Tarot), and Ober (Skat). The weight of the Rank determines how a Card is sorted relative to
/// it's Suit.
///
/// There are four ways to instantiate a Rank, each of them having advantages:
///
/// # As an *instance* variable
/// ```
/// let ace = cardpack::Rank {
///     weight: 1,
///     name: cardpack::fluent_name::FluentName::new(cardpack::ACE),
/// };
/// ```
/// This gives you maximum flexibility. Since the value of the Ace is 1, it will be sorted
/// at the and of a Suit (unless there are any Cards with negative weights).
///
/// # Rank::new() with a value string
/// ```
/// let king = cardpack::Rank::new(cardpack::KING);
/// ```
/// This sets the weight for the Rank based upon the default value as set in its fluent template
/// entry.
///
/// # Rank::new_with_weight()
/// ```
/// let king = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
/// ```
/// Overrides the default weight for a Rank.
///
/// # Rank::from_array()
/// ```
/// let ranks: Vec<cardpack::Rank> = cardpack::Rank::from_array(&[cardpack::ACE, cardpack::TEN,]);
/// ```
/// Returns a Vector of Ranks with their weights determined by the order they're passed in, high to
/// low. This facilitates the easy creation of custom decks, such as pinochle.
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank {
    /// Used by the Pile struct to sort Cards by their Suit and Rank.
    pub weight: isize,
    pub name: FluentName,
}

impl Rank {
    /// Returns a Rank, determining its weight by the default weight value for its name set in the
    /// fluent templates. For instance, if you look in `src/fluent/locales/core.ftl you will see
    /// that the default weight for an Ace is 14. This will mean that when a pile of cards is sorted
    /// that it will be at the top of a standard French Deck where the Ace is high.
    ///
    /// ## Usage
    /// ```
    /// let king = cardpack::Rank::new(cardpack::HERMIT);
    /// ```
    pub fn new(name: &'static str) -> Rank {
        let name = FluentName::new(name);
        Rank {
            weight: name.default_weight(),
            name,
        }
    }

    /// Returns a Rank instance with the passed in name and weight, overriding the default value
    /// set in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// let king = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
    /// ```
    pub fn new_with_weight(name: &'static str, weight: isize) -> Rank {
        Rank {
            weight,
            name: FluentName::new(name),
        }
    }

    /// ## Usage
    /// ```
    /// let ranks: Vec<cardpack::Rank> = cardpack::Rank::from_array(&[
    ///     cardpack::ACE, cardpack::TEN, cardpack::KING,
    ///     cardpack::QUEEN, cardpack::JACK, cardpack::NINE]);
    /// ```
    /// Returns a Vector of Ranks with their weights determined by the order they're passed in, high to
    /// low. This facilitates the easy creation of custom decks, such as for pinochle.
    pub fn from_array(s: &[&'static str]) -> Vec<Rank> {
        let mut v: Vec<Rank> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let weight = (s.len() + 1) - i;
            v.push(Rank::new_with_weight(elem, weight as isize));
        }
        v
    }

    pub fn generate_canasta_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            TWO, ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE,
        ])
    }

    pub fn generate_euchre_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, KING, QUEEN, JACK, TEN, NINE])
    }

    pub fn generate_french_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO,
        ])
    }

    pub fn generate_pinochle_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, TEN, KING, QUEEN, JACK, NINE])
    }

    pub fn generate_major_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            FOOL, MAGICIAN, PRIESTESS, EMPRESS, EMPEROR, HIEROPHANT, LOVERS, CHARIOT, STRENGTH,
            HERMIT, FORTUNE, JUSTICE, HANGED, DEATH, TEMPERANCE, DEVIL, TOWER, STAR, MOON, SUN,
            JUDGEMENT, WORLD,
        ])
    }

    pub fn generate_minor_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            KING, QUEEN, KNIGHT, PAGE, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ACE,
        ])
    }

    pub fn generate_skat_ranks() -> Vec<Rank> {
        Rank::from_array(&[DAUS, KING, OBER, UNTER, TEN, NINE, EIGHT, SEVEN])
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.index(&US_ENGLISH))
    }
}

impl Named for Rank {
    fn name(&self) -> &str {
        self.name.name()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use crate::{GERMAN, US_ENGLISH};

    #[test]
    fn display() {
        assert_eq!("Rang: A", format!("Rang: {}", Rank::new(ACE)));
    }

    #[test]
    fn get_index() {
        let queen = Rank::new(QUEEN);

        assert_eq!("Q".to_string(), queen.name.index_default());
        assert_eq!("D".to_string(), queen.name.index(&GERMAN));
    }

    #[test]
    fn get_long() {
        let ace = Rank::new(ACE);

        assert_eq!("Ace".to_string(), ace.name.long(&US_ENGLISH));
        assert_eq!("Ass".to_string(), ace.name.long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::new(KING).to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rank {
            weight: 9,
            name: FluentName::new(NINE),
        };

        assert_eq!(expected, Rank::new(NINE));
    }

    #[test]
    fn new__tarot() {
        let hermit = Rank::new(HERMIT);

        assert_eq!(Rank::new(HERMIT), hermit)
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Rank::new_with_weight(NINE, 3),
            Rank::new_with_weight(NINE, 4)
        );
        assert_ne!(
            Rank::new_with_weight(TEN, 4),
            Rank::new_with_weight(NINE, 4)
        );
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(KING, 3));
        expected.push(Rank::new_with_weight(QUEEN, 2));

        assert_eq!(expected, Rank::from_array(&[KING, QUEEN]));
    }

    #[test]
    fn generate_canasta_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(TWO, 14));
        expected.push(Rank::new_with_weight(ACE, 13));
        expected.push(Rank::new_with_weight(KING, 12));
        expected.push(Rank::new_with_weight(QUEEN, 11));
        expected.push(Rank::new_with_weight(JACK, 10));
        expected.push(Rank::new_with_weight(TEN, 9));
        expected.push(Rank::new_with_weight(NINE, 8));
        expected.push(Rank::new_with_weight(EIGHT, 7));
        expected.push(Rank::new_with_weight(SEVEN, 6));
        expected.push(Rank::new_with_weight(SIX, 5));
        expected.push(Rank::new_with_weight(FIVE, 4));
        expected.push(Rank::new_with_weight(FOUR, 3));
        expected.push(Rank::new_with_weight(THREE, 2));

        assert_eq!(expected, Rank::generate_canasta_ranks());
    }

    #[test]
    fn generate_euchre_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(ACE, 7));
        expected.push(Rank::new_with_weight(KING, 6));
        expected.push(Rank::new_with_weight(QUEEN, 5));
        expected.push(Rank::new_with_weight(JACK, 4));
        expected.push(Rank::new_with_weight(TEN, 3));
        expected.push(Rank::new_with_weight(NINE, 2));

        assert_eq!(expected, Rank::generate_euchre_ranks());
    }

    #[test]
    fn generate_french_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new(ACE));
        expected.push(Rank::new(KING));
        expected.push(Rank::new(QUEEN));
        expected.push(Rank::new(JACK));
        expected.push(Rank::new(TEN));
        expected.push(Rank::new(NINE));
        expected.push(Rank::new(EIGHT));
        expected.push(Rank::new(SEVEN));
        expected.push(Rank::new(SIX));
        expected.push(Rank::new(FIVE));
        expected.push(Rank::new(FOUR));
        expected.push(Rank::new(THREE));
        expected.push(Rank::new(TWO));

        assert_eq!(expected, Rank::generate_french_ranks());
    }

    #[test]
    fn generate_pinochle_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(ACE, 7));
        expected.push(Rank::new_with_weight(TEN, 6));
        expected.push(Rank::new_with_weight(KING, 5));
        expected.push(Rank::new_with_weight(QUEEN, 4));
        expected.push(Rank::new_with_weight(JACK, 3));
        expected.push(Rank::new_with_weight(NINE, 2));

        assert_eq!(expected, Rank::generate_pinochle_ranks());
    }

    #[test]
    fn generate_major_arcana_ranks() {
        let major = Rank::generate_major_arcana_ranks();

        assert_eq!(22, major.len());
    }

    #[test]
    fn generate_minor_arcana_ranks() {
        let ex: Vec<Rank> = Rank::from_array(&[
            KING, QUEEN, KNIGHT, PAGE, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ACE,
        ]);

        assert_eq!(ex, Rank::generate_minor_arcana_ranks());
    }

    #[test]
    fn revise_value() {
        let mut ace = Rank::new(ACE);
        assert_eq!(14, ace.weight);

        ace.weight = 3;

        assert_eq!(3, ace.weight);
    }
}
