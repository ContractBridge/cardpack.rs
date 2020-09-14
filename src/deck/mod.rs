pub(crate) mod suit;
mod suit_letter;
mod suit_name;
mod suit_symbol;
pub(crate) mod rank;
mod rank_short;
mod rank_name;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, GERMAN, US_ENGLISH};
use crate::deck::suit::Suit;
use crate::deck::rank::Rank;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    pub value: isize,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new<S: std::clone::Clone>(rang: S, anzug: S) -> Card
    where
        S: Into<String>,
    {
        let a = Suit::new(anzug);
        let r = Rank::new(rang);
        let wert = Card::determine_value(&a, &r);
        Card {
            value: wert,
            suit: a,
            rank: r,
        }
    }

    pub fn new_from_structs(rank: Rank, suit: Suit) -> Card {
        let value = Card::determine_value(&suit, &rank);
        Card { value, rank, suit }
    }

    fn determine_value(anzug: &Suit, rang: &Rank) -> isize {
        (anzug.value * 100) + rang.value
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rank.to_locale_string(&lid);
        let anzug = self.suit.letter.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

impl ToLocaleString for Card {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rank.to_locale_string(&lid);
        let anzug = self.suit.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn new() {
        let expected = Card {
            value: 414,
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(expected, Card::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            value: 414,
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(
            expected,
            Card::new_from_structs(Rank::new("ace"), Suit::new("spades"))
        );
    }

    #[test]
    fn to_string_by_locale() {
        let karte = Card::new("queen", "clubs");

        assert_eq!(karte.to_locale_string(&GERMAN), "D♣".to_string());
    }

    #[test]
    fn to_txt_string() {
        let karte = Card::new("queen", "clubs");

        assert_eq!(karte.to_txt_string(&GERMAN), "DK".to_string());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        Deck::new_from_vector(Vec::new())
    }

    pub fn new_from_vector(v: Vec<Card>) -> Deck {
        Deck(v)
    }

    pub fn add(&mut self, elem: Card) {
        self.0.push(elem);
    }

    pub fn all(&self) -> &Vec<Card> {
        &self.0
    }

    // Appends a clone of the passed in Karten struct.
    pub fn append(&mut self, other: &Deck) {
        self.0.append(&mut other.0.clone());
    }

    // Takes a reference to the prepended entity, clones it, appends the original to the passed in
    // entity, and replaces the original with the new one.
    pub fn prepend(&mut self, other: &Deck) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    pub fn contains(&self, karte: &Card) -> bool {
        self.0.contains(karte)
    }

    pub fn demo(&self) {
        print!("   Long in English and German:\n");
        for karte in self.values() {
            let anzugname = karte.suit.name.to_locale_string(&GERMAN);
            let suitname = karte.suit.name.to_locale_string(&US_ENGLISH);
            let rangname = karte.rank.name.to_locale_string(&GERMAN);
            let rankname = karte.rank.name.to_locale_string(&US_ENGLISH);
            println!("      {} of {} ", rankname, suitname);
            println!("      {} von {} ", rangname, anzugname);
        }

        println!();
        print!("   Short With Symbols:           ");
        for karte in self.values() {
            print!("{} ", karte);
        }

        println!();
        print!("   Short With Symbols in German: ");
        for karte in self.values() {
            print!("{} ", karte.to_locale_string(&GERMAN));
        }

        println!();
        print!("   Short With Letters:           ");
        for karte in self.values() {
            print!("{} ", karte.to_txt_string(&US_ENGLISH));
        }

        println!();
        print!("   Short With Letters in German: ");
        for karte in self.values() {
            print!("{} ", karte.to_txt_string(&GERMAN));
        }

        println!();
        print!("   Shuffle Deck:                 ");
        let mut mische = self.mischen();
        for karte in mische.values() {
            print!("{} ", karte.to_locale_string(&US_ENGLISH));
        }

        println!();
        print!("   Sort Deck:                    ");
        mische.sort();
        for karte in mische.values() {
            print!("{} ", karte.to_locale_string(&US_ENGLISH));
        }

        println!();
    }

    pub fn draw(&mut self, x: usize) -> Option<Deck> {
        if x > self.len() {
            None
        } else {
            let mut karten = Deck::new();
            for _ in 0..x {
                karten.add(self.draw_first().unwrap());
            }
            Some(karten)
        }
    }

    pub fn draw_first(&mut self) -> Option<Card> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    pub fn draw_last(&mut self) -> Option<Card> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(self.len() - 1)),
        }
    }

    pub fn first(&self) -> Option<&Card> {
        self.0.first()
    }

    pub fn get(&self, index: usize) -> Option<&Card> {
        self.0.get(index)
    }

    pub fn get_random(&self) -> Option<&Card> {
        self.0.choose(&mut rand::thread_rng())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn last(&self) -> Option<&Card> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn position(&self, karte: &Card) -> Option<usize> {
        self.0.iter().position(|k| k == karte)
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    pub fn remove_karte(&mut self, karte: &Card) -> Option<Card> {
        let position = self.position(karte);
        match position {
            None => None,
            _ => Some(self.0.remove(position.unwrap())),
        }
    }

    pub fn mischen(&self) -> Deck {
        let mut mischte = self.clone();
        mischte.0.shuffle(&mut thread_rng());
        mischte
    }

    pub fn sort(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    pub fn values(&self) -> impl Iterator<Item = &Card> {
        self.0.iter()
    }

    pub fn jokers() -> Deck {
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        Deck::new_from_vector(vec![big_joker, little_joker])
    }

    pub fn french_deck() -> Deck {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_french_ranks();

        let mut karten: Deck = Deck::new();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }
        karten
    }

    pub fn pinochle_deck() -> Deck {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_pinochle_ranks();

        let mut karten: Deck = Deck::new();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.add(Card::new_from_structs(rank.clone(), suit.clone()));
                karten.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }
        karten
    }

    pub fn spades_deck() -> Deck {
        let mut deck = Deck::french_deck();
        deck.remove_karte(&Card::new("two", "clubs"));
        deck.remove_karte(&Card::new("two", "diamonds"));
        let jokers = Deck::jokers();

        deck.prepend(&jokers);
        deck
    }

    pub fn tarot_deck() -> Deck {
        let arcana_suits = Suit::generate_arcana_suits();
        let mut arcana_suits_enumerator = arcana_suits.iter().enumerate();
        let major_arcana_ranks = Rank::generate_major_arcana_ranks();
        let minor_arcana_ranks = Rank::generate_minor_arcana_ranks();

        let mut karten: Deck = Deck::new();

        let (_, major_arcana_suit) = arcana_suits_enumerator.next().unwrap();

        // Generate Major Arcana
        for (_, rank) in major_arcana_ranks.iter().enumerate() {
            karten.add(Card::new_from_structs(
                rank.clone(),
                major_arcana_suit.clone(),
            ));
        }

        // Generate Minor Arcana
        for (_, suit) in arcana_suits_enumerator {
            for (_, rank) in minor_arcana_ranks.iter().enumerate() {
                karten.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }

        karten
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod karten_tests {
    use super::*;

    #[test]
    fn new_all_add_new_from_vector() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut expected = Deck::new();
        expected.add(qclubs.clone());
        expected.add(qhearts.clone());

        let actual = Deck::new_from_vector(vec![qclubs, qhearts]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn append() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        let mut to_deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Deck::jokers();
        let expected = Deck::new_from_vector(vec![qclubs, qhearts, big_joker, little_joker]);

        to_deck.append(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn prepend() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        let mut to_deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Deck::new_from_vector(vec![big_joker.clone(), little_joker.clone()]);
        let expected = Deck::new_from_vector(vec![big_joker, little_joker, qclubs, qhearts]);

        to_deck.prepend(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn contains() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(deck.contains(&qclubs));
        assert!(deck.contains(&qhearts));
    }

    #[test]
    fn draw() {
        let mut zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let qspades = Card::new("queen", "spades");
        let mut deck =
            Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone(), qspades.clone()]);

        assert!(zero.draw(2).is_none());
        assert_eq!(
            deck.draw(2).unwrap(),
            Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()])
        );
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_first() {
        let mut zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_first().is_none());
        assert_eq!(deck.draw_first().unwrap(), qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_last() {
        let mut zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_last().is_none());
        assert_eq!(deck.draw_last().unwrap(), qhearts);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn first() {
        let zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.first().is_none());
        assert_eq!(deck.first().unwrap(), &qclubs);
    }

    #[test]
    fn get() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let gotten = deck.get(1);

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn get_random() {
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qhearts.clone()]);

        let gotten = deck.get_random();

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn last() {
        let zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.last().is_none());
        assert_eq!(deck.last().unwrap(), &qhearts);
    }

    #[test]
    fn len() {
        let zero = Deck::new();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(zero.len(), 0);
        assert_eq!(deck.len(), 2);
    }

    #[test]
    fn position() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(0, deck.position(&qclubs).unwrap());
        assert_eq!(1, deck.position(&qhearts).unwrap());
    }

    #[test]
    fn remove() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove(0);

        assert_eq!(removed, qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn remove_karte() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Deck::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove_karte(&qclubs);

        assert_eq!(removed.unwrap(), qclubs);
        assert!(deck.contains(&qhearts));
        assert!(!deck.contains(&qclubs));
    }

    #[test]
    fn sort() {
        let french_deck = Deck::french_deck();

        let mut shuffled = french_deck.mischen();
        shuffled.sort();

        assert_eq!(french_deck, shuffled);
    }

    #[test]
    fn spades_deck() {
        let deck = Deck::spades_deck();

        assert!(!deck.contains(&Card::new("two", "clubs")));
        assert!(!deck.contains(&Card::new("two", "diamonds")));
        assert!(deck.contains(&Card::new("two", "spades")));
    }
}
