# cardpack.rs

[![Build Status](https://api.travis-ci.com/ContractBridge/cardpack.rs.svg?branch=main)](https://travis-ci.com/github/ContractBridge/cardpack.rs)

Generic pack of cards library written in Rust. The goals of the library include:

* Various types of decks of cards.
* Internationalization support.
* Ability to create custom sorts for a specific pack of cards.

## Usage

```rust
fn main() {
    let pack = cardpack::Pack::french_deck();

    let mut shuffled = pack.cards().shuffle();
    let sb = shuffled.draw(2).unwrap();
    let bb = shuffled.draw(2).unwrap();

    println!("small blind: {}", sb.by_symbol_index());
    println!("big blind:   {}", bb.by_symbol_index());

    println!();
    println!("flop : {}", shuffled.draw(3).unwrap().by_symbol_index());
    println!("turn : {}", shuffled.draw(1).unwrap().by_symbol_index());
    println!("river: {}", shuffled.draw(1).unwrap().by_symbol_index());
}
```

## Examples

The library has several demo programs in the examples directory.

For the traditional French Deck:

```
$> cargo run --example traditional
Long in English and German:
      Ace of Spades
      Ass von Spaten
      King of Spades
      König von Spaten
      Queen of Spades
      Dame von Spaten
...
   Short With Symbols:           A♠ K♠ Q♠ J♠ 10♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ 10♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ 10♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ 10♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
   Short With Symbols in German: A♠ K♠ D♠ B♠ 10♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ D♥ B♥ 10♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ D♦ B♦ 10♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ D♣ B♣ 10♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
   Short With Letters:           AS KS QS JS 10S 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH 10H 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD 10D 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC 10C 9C 8C 7C 6C 5C 4C 3C 2C
   Short With Letters in German: AS KS DS BS 10S 9S 8S 7S 6S 5S 4S 3S 2S AH KH DH BH 10H 9H 8H 7H 6H 5H 4H 3H 2H AD KD DD BD 10D 9D 8D 7D 6D 5D 4D 3D 2D AK KK DK BK 10K 9K 8K 7K 6K 5K 4K 3K 2K
   Shuffle Deck:                 J♠ 10♥ 8♣ 7♠ 4♣ K♠ 7♦ Q♠ 6♠ 3♠ 7♥ 4♥ Q♥ J♦ 5♠ 9♠ A♦ 3♦ 10♠ 5♦ 6♥ Q♣ 6♦ 8♥ 5♣ 3♣ A♥ 10♣ 8♠ J♥ 5♥ 2♦ 6♣ 9♦ K♦ Q♦ 2♥ A♠ 7♣ K♣ J♣ 2♠ 4♠ 3♥ 9♣ 2♣ 4♦ K♥ A♣ 9♥ 8♦ 10♦
   Sort Deck:                    A♠ K♠ Q♠ J♠ 10♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ 10♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ 10♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ 10♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
```

Also included are example programs for `spades`, `pinochle`, `skat`, and `tarot`.

## Details

The goal of this library is to be able to support the creation of card
decks of various sizes and suits. Out of the box, the library supports:

* [French Deck](https://en.wikipedia.org/wiki/French_playing_cards)
  * [Pinochle](https://en.wikipedia.org/wiki/Pinochle#Deck)
  * [Spades](https://en.wikipedia.org/wiki/Spades_(card_game)#General_overview) with [Jokers](https://en.wikipedia.org/wiki/Joker_(playing_card))
  * [Standard 52](https://en.wikipedia.org/wiki/Standard_52-card_deck)
* [Skat](https://en.wikipedia.org/wiki/Skat_(card_game)#Deck)
* [Tarot](https://en.wikipedia.org/wiki/Tarot#Tarot_gaming_decks) with [Major](https://en.wikipedia.org/wiki/Major_Arcana) and [Minor](https://en.wikipedia.org/wiki/Minor_Arcana) Arcana

The project takes advantage of [Project Fluent](https://www.projectfluent.org/)'s
[Rust](https://github.com/projectfluent/fluent-rs) support to offer
internationalization. Current languages supported are
[English](src/fluent/locales/en-US/french-deck.ftl) and
[German](src/fluent/locales/de/french-deck.ftl).

Other possibilities include:

* French Deck
  * [Canasta](https://en.wikipedia.org/wiki/Canasta#Cards_and_deal)
  * [Euchre](https://en.wikipedia.org/wiki/Euchre)

## Responsibilities

* Represent a specific type of card deck.
* Validate that a collection of cards is valid for that type of deck.
* Create a textual representation of a deck that can be serialized and deserialized.
* Shuffle a deck
* Verify that a specific card is playable given a set of discards.
* Determine if two deck types are translatable.

## References

* [Card games in Germany](https://www.pagat.com/national/germany.html)
* [Playing cards in Unicode](https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)

### Other Deck of Cards Libraries

* [ascclemens/cards](https://github.com/ascclemens/cards)
* [locka99/deckofcards-rs](https://github.com/locka99/deckofcards-rs)
* [vsupalov/cards-rs](https://github.com/vsupalov/cards-rs)
* Tarot Libraries
  * [lawreka/ascii-tarot](https://github.com/lawreka/ascii-tarot)
  * [pietdaniel/tarot](https://github.com/pietdaniel/tarot)
  * [jeremytarling/ruby-tarot](https://github.com/jeremytarling/ruby-tarot)

## Dependencies

* [Fluent Templates](https://github.com/XAMPPRocky/fluent-templates)
  * [Project Fluent](https://www.projectfluent.org/)
