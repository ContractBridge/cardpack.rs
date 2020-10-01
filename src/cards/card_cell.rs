use std::cell::{Cell, RefCell};

use crate::cards::card::Card;

/// The structure of this struct is to deal with the issue that RefCell.take() is only
/// available in unstable. Once that feature has been merged in we can eliminate the Cell
/// and just have it as a newtype struct.
///
/// See https://github.com/rust-lang/rust/issues/71395
///
// #[derive(Debug)]
pub struct CardCell {
    cell: Cell<Card>,
    aligned: Cell<bool>,
    card: RefCell<Card>,
}

impl CardCell {
    pub fn new(card: Card) -> CardCell {
        CardCell {
            /// The internal container for the actual Card. When it is dealt it is replaced with
            /// a blank Card.
            cell: Cell::new(card.clone()),
            /// Tracks the state of the CardCell. True if the Card has not been dealt.
            aligned: Cell::new(true),
            /// A reference to what the card is in the Cell. A Card from a CardCell can only be
            /// returned if it matches this card.
            card: RefCell::new(card),
        }
    }

    pub fn deal(&self) -> Card {
        self.aligned.take();
        self.cell.take()
    }

    pub fn is_there(&self) -> bool {
        let is_there = self.aligned.take();
        self.aligned.replace(is_there.clone());
        is_there
    }
}

// impl PartialEq for CardCell {
//     fn eq(&self, other: &Self) -> bool {
//         let taken = self.0.take();
//         self.0.replace(taken.clone());
//
//         self.path == other.path
//     }
// }

#[cfg(test)]
#[allow(non_snake_case)]
mod card_cell_tests {
    use super::*;
    use crate::{SPADES, TWO};

    #[test]
    fn new() {
        let deuce = Card::new(TWO, SPADES);
        let _ = CardCell {
            cell: Cell::new(deuce.clone()),
            aligned: Cell::new(true),
            card: RefCell::new(deuce.clone()),
        };

        let _ = CardCell::new(deuce);

        // assert_eq!(expected, actual)
    }

    #[test]
    fn deal() {
        let deuce = Card::new(TWO, SPADES);
        let cc = CardCell::new(deuce.clone());

        let actual = cc.deal();

        assert_eq!(deuce, actual);
        assert!(!cc.is_there())
    }

    #[test]
    fn is_there() {
        let cc = CardCell::new(Card::new(TWO, SPADES));

        assert!(cc.is_there())
    }
}
