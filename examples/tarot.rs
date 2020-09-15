fn main() {
    let deck = spielkartenlib::Pack::tarot_deck();

    println!("Tarot Deck");
    display(&deck);

    println!();
    println!("Tarot Deck Shuffled");
    let mut shuffled = deck.shuffle();
    display(&shuffled);

    println!();
    println!("Tarot Deck Sorted");
    shuffled.sort();
    display(&shuffled);
}

fn display(deck: &spielkartenlib::Pack) {
    for card in deck.values() {
        let suitname = card.suit.name.to_string();

        let rankname = card.rank.name.to_string();
        if suitname == "Major Arcana".to_string() {
            println!("      {}", rankname);
        } else {
            println!("      {} of {} ", rankname, suitname);
        }
    }
}
