// Card Related Structures

#[derive(Debug, PartialEq, Clone)]
pub enum Suit {
    Diamond,
    Heart,
    Club,
    Spade,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rank {
    Ace,
    One,
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
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Rank::Ace => write!(f, "Ace"),
            Rank::One => write!(f, "One"),
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card(Suit, Rank);

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub type Deck = Vec<Card>;

// Game Related Structures

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    name: String,
    chips: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerMove {
    Fold,
    Check,
    Bet(usize),
    Raise(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub struct HoleCards(Card, Card);

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerState {
    WaitingToBeDealt,
    Dealt(HoleCards),
    Active(HoleCards),
    Folded,
}

#[derive(Clone)]
pub struct Hand {
    players: Vec<(Player, PlayerState)>,
    deck: Deck,
    pot: usize,
    // community_cards: Vec<Card>,
}

pub fn new_hand(players: Vec<Player>, deck: Deck) -> Hand {
    let mut vector: Vec<(Player, PlayerState)> = vec![];
    for p in players {
        vector.push((p, PlayerState::WaitingToBeDealt));
    }
    Hand {
        players: vector,
        pot: 0,
        deck,
    }
}

pub fn deal(hand: Hand) -> Hand {
    let players = hand.players.clone();
    let mut new_players: Vec<(Player, PlayerState)> = vec![];
    let mut new_deck = hand.deck;
    let mut i: u32 = 0;

    for p in players {
        if i == 0 {
            new_players.push((p.0, PlayerState::Active(
                HoleCards(
                    new_deck.remove(0),
                    new_deck.remove(0)
                )
            )))
        } else {
            new_players.push((p.0, PlayerState::Dealt(
                HoleCards(
                    new_deck.remove(0),
                    new_deck.remove(0)
                )
            )))
        }
        i += 1;
    }

    Hand {
        players: new_players,
        pot: hand.pot,
        deck: new_deck
    }
}

pub fn play(hand: Hand, mv: PlayerMove) -> Hand {
    match mv {
        PlayerMove::Bet(num) => {
            let mut new_pot = hand.pot.clone();
            new_pot += num;
            Hand { 
                players: hand.players, 
                deck: hand.deck, 
                pot: new_pot
            }
        },
        PlayerMove::Raise(num) => {
            let mut new_pot = hand.pot.clone();
            new_pot += num;
            Hand { 
                players: hand.players, 
                deck: hand.deck, 
                pot: new_pot
            }
        },
        PlayerMove::Fold => {
            let mut new_players = hand.players.clone();

            if let Some(first) = new_players.first_mut() {
                first.1 = PlayerState::Folded;
            }

            Hand {
                players: new_players,
                deck: hand.deck,
                pot: hand.pot
            }
        }
        _ => hand
    }
}

// Unused but potentially interesting follow-on structures

pub enum PlayerPosition {
    Button,
    SmallBlind,
    BigBlind,
}

pub enum Deal {
    Hole,
    Flop,
    River,
    Turn,
}

pub enum HandValues {
    HighCard(Card),
    Pair((Card, Card)),
    TwoPairs((Card, Card), (Card, Card)),
    ThreeOfAKind((Card, Card, Card)),
    Straight((Card, Card, Card, Card, Card)),
    Flush((Card, Card, Card, Card, Card)),
    FullHouse((Card, Card, Card, Card, Card)),
    FourOfAKind((Card, Card, Card, Card)),
    StraightFlush((Card, Card, Card, Card, Card)),
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suits_display_as_icons_and_ranks_as_text() {
        // Hint: The format! macro makes use of the Display Trait.
        // How can we display different values for different enumerations?
        assert_eq!("(♥, Ace)", format!("{}", Card(Suit::Heart, Rank::Ace)));
        assert_eq!("(♦, Ten)", format!("{}", Card(Suit::Diamond, Rank::Ten)));
        assert_eq!("(♣, Ace)", format!("{}", Card(Suit::Club, Rank::Ace)));
        assert_eq!("(♠, Jack)", format!("{}", Card(Suit::Spade, Rank::Jack)));
    }

    #[test]
    fn new_hand_results_in_all_players_waiting_and_pot_of_zero() {
        let hand = new_hand(
            vec![
                Player {
                    name: s("Will"),
                    chips: 10,
                },
                Player {
                    name: s("Jean"),
                    chips: 2,
                },
            ],
            simple_deck(),
        );

        assert_eq!(hand.pot, 0);
        assert_eq!(hand.deck, simple_deck());
        assert_eq!(hand.players.len(), 2);
        for player in hand.players.iter() {
            assert_eq!(player.1, PlayerState::WaitingToBeDealt);
        }
    }

    #[test]
    fn deal_provides_cards_from_deck_and_sets_first_player_active() {
        let will = Player {
            name: s("Will"),
            chips: 10,
        };
        let hand = new_hand(
            vec![
                will.clone(),
                Player {
                    name: s("Jean"),
                    chips: 2,
                },
            ],
            // Note: simple_deck just has some aces
            simple_deck(),
        );

        let hand = deal(hand);
        assert_eq!(
            hand.players.first().unwrap().1,
            PlayerState::Active(HoleCards(
                simple_deck()[0].clone(),
                simple_deck()[1].clone()
            )),
        );
        assert_eq!(
            hand.players.last().unwrap().1,
            PlayerState::Dealt(HoleCards(
                simple_deck()[2].clone(),
                simple_deck()[3].clone()
            )),
        );
    }

    #[test]
    fn check_leaves_pot_untouched() {
        let hand = new_hand(
            vec![
                Player {
                    name: s("Will"),
                    chips: 10,
                },
                Player {
                    name: s("Jean"),
                    chips: 2,
                },
            ],
            simple_deck(),
        );

        let hand = play(hand, PlayerMove::Check);
        assert_eq!(hand.pot, 0);

        // TODO: This should move the active player
        // assert_eq!(
        //     hand.players.first().unwrap().1,
        //     PlayerState::Dealt(HoleCards(
        //         simple_deck()[0].clone(),
        //         simple_deck()[1].clone()
        //     )),
        // );
        // assert_eq!(
        //     hand.players.last().unwrap().1,
        //     PlayerState::Active(HoleCards(
        //         simple_deck()[2].clone(),
        //         simple_deck()[3].clone()
        //     )),
        // );
    }

    #[test]
    fn bet_increases_the_pot() {
        let hand = new_hand(
            vec![
                Player {
                    name: s("Will"),
                    chips: 10,
                },
                Player {
                    name: s("Jean"),
                    chips: 2, // <--- loser
                },
            ],
            simple_deck(),
        );

        let hand = play(hand, PlayerMove::Bet(3));
        assert_eq!(hand.pot, 3);

        // TODO: This should move the active player and decrease from active players chips
    }

    #[test]
    fn raise_increases_the_pot() {
        let hand = new_hand(
            vec![
                Player {
                    name: s("Will"),
                    chips: 10,
                },
                Player {
                    name: s("Jean"),
                    chips: 2, // <--- loser
                },
            ],
            simple_deck(),
        );

        let hand = play(hand, PlayerMove::Raise(3));
        assert_eq!(hand.pot, 3);

        // TODO: This should move the active player and decrease from active players chips
    }

    #[test]
    fn fold_sets_the_player_state_to_folded() {
        let hand = new_hand(
            vec![
                Player {
                    name: s("Will"),
                    chips: 10,
                },
                Player {
                    name: s("Jean"),
                    chips: 2, // <--- loser
                },
            ],
            simple_deck(),
        );

        let hand = play(hand, PlayerMove::Fold);
        assert_eq!(hand.players.first().unwrap().1, PlayerState::Folded)

        // TODO: This should move the active player
    }

    fn s(s: &str) -> String {
        s.to_owned()
    }

    fn simple_deck() -> Deck {
        let mut deck = Deck::new();
        deck.push(Card(Suit::Heart, Rank::Ace));
        deck.push(Card(Suit::Diamond, Rank::Ace));
        deck.push(Card(Suit::Club, Rank::Ace));
        deck.push(Card(Suit::Spade, Rank::Ace));

        deck
    }
}
