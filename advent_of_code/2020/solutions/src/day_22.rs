use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    fmt::Display,
};

// obscure errors of you don't import these
use std::hash::Hash;
use std::hash::Hasher;

use crate::{AoCData, AoCResult};

#[derive(Debug, PartialEq)]
enum Winner {
    Player1,
    Player2,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<VecDeque<usize>>);

fn combat(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) -> Winner {
    while deck1.len() > 0 && deck2.len() > 0 {
        let card1 = deck1
            .pop_front()
            .expect("Tried to show card from empty deck1");
        let card2 = deck2
            .pop_front()
            .expect("Tried to show card from empty deck2");

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    if deck1.len() > 0 {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn recursive_combat(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) -> Winner {
    // hashset tuples (player1_deck_hashed, player2_deck_hashed)
    // question states: if there was a previous round in this game that had exactly the same cards in the same order in the same players' decks
    // conclusion: both player1 and player2 have to have the exact same deck as some round before.
    let mut previous_rounds: HashSet<(u64, u64)> = HashSet::new();

    while deck1.len() > 0 && deck2.len() > 0 {
        let deck1_hash = hash_deck(deck1);
        let deck2_hash = hash_deck(deck2);
        let deck_hashes = (deck1_hash, deck2_hash);
        // check if decks were exactly the same earlier in the game
        if previous_rounds.contains(&deck_hashes) {
            return Winner::Player1;
        }

        // add hashes to set
        previous_rounds.insert(deck_hashes);

        // deal 1 card each
        let card1 = deck1
            .pop_front()
            .expect("Tried to show card from empty deck1");
        let card2 = deck2
            .pop_front()
            .expect("Tried to show card from empty deck2");

        let winner = if deck1.len() >= card1 && deck2.len() >= card2 {
            // (the quantity of cards copied is equal to the number on the card they drew to trigger the sub-game)
            // VecDeque can't be sliced, so we make our subdecks the long way around, by cloning the entire deck and truncating it
            let mut sub_deck1 = deck1.clone();
            sub_deck1.truncate(card1);
            let mut sub_deck2 = deck2.clone();
            sub_deck2.truncate(card2);
            recursive_combat(&mut sub_deck1, &mut sub_deck2)
        } else {
            if card1 > card2 {
                Winner::Player1
            } else {
                // also fires if the values are equal since the question doesn't cover that possibility
                Winner::Player2
            }
        };
        match winner {
            Winner::Player1 => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            }
            Winner::Player2 => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }
    if deck1.len() > 0 {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn hash_deck(deck: &VecDeque<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    deck.clone().hash(&mut hasher);
    hasher.finish()
}

fn get_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .enumerate()
        .fold(0, |acc, (idx, num)| acc + num * (deck.len() - idx))
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut result: Vec<VecDeque<usize>> = Vec::new();
        let players: Vec<&str> = input.split("\n\n").collect();
        for player in players {
            let mut deck = VecDeque::new();
            let parts: Vec<&str> = player.splitn(2, "\n").collect();
            for num in parts[1].lines() {
                deck.push_back(num.parse().unwrap())
            }
            result.push(deck);
        }
        Ok(Self(result))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut player1_deck = self.0[0].clone();
        let mut player2_deck = self.0[1].clone();
        let winning_player = combat(&mut player1_deck, &mut player2_deck);
        let winning_deck = match winning_player {
            Winner::Player1 => player1_deck,
            Winner::Player2 => player2_deck,
        };

        Ok(get_score(&winning_deck))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut player1_deck = self.0[0].clone();
        let mut player2_deck = self.0[1].clone();
        let winning_player = recursive_combat(&mut player1_deck, &mut player2_deck);
        let winning_deck = match winning_player {
            Winner::Player1 => player1_deck,
            Winner::Player2 => player2_deck,
        };

        Ok(get_score(&winning_deck))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "306");
    }

    #[test]
    fn part_2() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "291");
    }
}
