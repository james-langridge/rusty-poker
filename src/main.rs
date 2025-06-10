#![warn(clippy::all)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    fn is_face_card(&self) -> bool {
        match self.rank {
            Rank::Jack | Rank::Queen | Rank::King => true,
            _ => false,
        }
    }
}

fn main() {
    let ace_of_spades = Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
    };

    let king_of_hearts = Card {
        rank: Rank::King,
        suit: Suit::Hearts,
    };

    let another_ace = Card {
        rank: Rank::Ace,
        suit: Suit::Diamonds,
    };

    if ace_of_spades.rank == another_ace.rank {
        println!("Both cards are aces!");
    }

    let my_card = ace_of_spades;

    println!("Original: {:?}", ace_of_spades);
    println!("Copy: {:?}", my_card);

    if king_of_hearts.is_face_card() {
        println!("The king of Heards is indeed a face card.");
    }
}
