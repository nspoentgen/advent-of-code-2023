use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;
use crate::hand::HandType::*;
use crate::hand::Card::*;

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Default, Hash, Debug)]
pub enum Card
{
    Joker,
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Default, Hash, Debug)]
pub enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Default)]
pub struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u32
}

impl Hand {
    pub fn get_cards(&self) -> &[Card; 5] {
        return &self.cards;
    }

    pub fn get_hand_type(&self) -> &HandType {
        return &self.hand_type;
    }

    pub fn get_bid(&self) -> &u32 {
        return &self.bid;
    }

    pub fn new(cards: [Card; 5], bid: u32) -> Self {
        Self {
            cards,
            hand_type: Hand::calculate_hand_type(&cards),
            bid
        }
    }

    fn calculate_hand_type(cards: &[Card; 5]) -> HandType {
        let mut possible_hand_types = Vec::<HandType>::new();
        let non_joker_cards = &cards
            .iter()
            .filter(|x| **x != Joker)
            .map(|x| *x)
            .collect::<Vec<Card>>();

        //Add joker cases
        if non_joker_cards.len() < cards.len() {
            let choices = non_joker_cards.iter().unique().map(|x| *x).collect::<Vec<Card>>();
            let num_jokers = cards.len() - non_joker_cards.len();
            let joker_cartesian_product_iter = (1..=num_jokers)
                .map(|_| &choices)
                .multi_cartesian_product()
                .map(|x| x)
                .collect::<Vec<Vec<&Card>>>();

            for joker_converted_subset in joker_cartesian_product_iter {
                let mut test_hand = non_joker_cards.clone();

                //This is technically wrong for perfect results because we are not doing an inline replaced of the
                //jokers. We are appending them to the end. However for the purposes of deciding the hand type,
                //exact ordering does not matter.
                test_hand.extend(joker_converted_subset);
                possible_hand_types.push(Self::calculate_non_joker_hand_type(&mut (&test_hand).into_iter()));
            }
        }

        //Add nominal case
        possible_hand_types.push(Self::calculate_non_joker_hand_type(&mut cards.into_iter()));

        return *possible_hand_types.iter().max().unwrap();
    }

    fn calculate_non_joker_hand_type<'a, T: Iterator<Item=&'a Card>>(cards: &mut T) -> HandType {
        let mut card_counts = HashMap::<Card,u32>::new();
        for card in cards {
            card_counts.insert(*card, if card_counts.contains_key(card) { card_counts[card] + 1 } else { 1 });
        }

        return if card_counts.iter().any(|x| *x.1 == 5u32) {
            FiveOfAKind
        } else if card_counts.iter().any(|x| *x.1 == 4u32) {
            FourOfAKind
        } else if card_counts.iter().filter(|x| *x.1 == 3u32).count() == 1 &&
            card_counts.iter().filter(|x| *x.1 == 2u32).count() == 1 {
            FullHouse
        } else if card_counts.iter().any(|x| *x.1 == 3u32) {
            ThreeOfAKind
        } else if card_counts.iter().filter(|x| *x.1 == 2u32).count() == 2 {
            TwoPair
        } else if card_counts.iter().filter(|x| *x.1 == 2u32).count() == 1 {
            OnePair
        }  else {
            HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type < other.hand_type {
            return Some(Less)
        } else if self.hand_type > other.hand_type {
            return Some(Greater)
        } else {
            for card_pair in zip(&self.cards, &other.cards) {
                if card_pair.0 < card_pair.1 {
                    return Some(Less);
                } else if card_pair.0 > card_pair.1 {
                    return Some(Greater);
                }
            }

            return Some(Equal);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand_type == other.hand_type &&
            zip(&self.cards, &other.cards)
                .into_iter()
                .all(|x| x.0 == x.1);
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return Hand::partial_cmp(self, other).unwrap();
    }
}
