use std::collections::VecDeque;

#[derive(Clone)]
pub struct SpaceCards {
    players: [Player; 2],
}

#[allow(unused)]
#[derive(Clone)]
pub struct Player {
    label: String,
    deck: VecDeque<u32>,
}

impl From<&str> for Player {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let label = lines.next().unwrap().trim_end().into();
        let deck = lines
            .map(str::trim_end)
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();
        Self { label, deck }
    }
}

impl Player {
    pub fn new(label: &str, deck: &[u32]) -> Self {
        let label = label.into();
        let deck = deck.iter().copied().collect();
        Self { label, deck }
    }
}

pub enum GameResult {
    OneRound(usize),
    GameOver(usize),
}

impl SpaceCards {
    pub fn new(input: &str) -> Self {
        let (player1, player2) = {
            let mut parts = input.split("\n\n");
            (
                Player::from(parts.next().unwrap()),
                Player::from(parts.next().unwrap()),
            )
        };
        Self {
            players: [player1, player2],
        }
    }

    fn part1_next_round(&mut self) -> GameResult {
        let cards = (
            self.players[0].deck.pop_front().unwrap(),
            self.players[1].deck.pop_front().unwrap(),
        );
        if cards.0 > cards.1 {
            self.players[0].deck.push_back(cards.0);
            self.players[0].deck.push_back(cards.1);
            return GameResult::OneRound(0);
        }
        self.players[1].deck.push_back(cards.1);
        self.players[1].deck.push_back(cards.0);
        GameResult::OneRound(1)
    }

    pub fn part1_game(&mut self) -> usize {
        loop {
            if let GameResult::OneRound(winner) = self.part1_next_round() {
                if self.players.iter().any(|p| p.deck.is_empty()) {
                    return winner;
                }
            }
        }
    }

    fn part2_next_round<'a>(
        &'a mut self,
        history: &'a mut Vec<(Vec<u32>, Vec<u32>)>,
    ) -> GameResult {
        self.players[0].deck.make_contiguous();
        self.players[1].deck.make_contiguous();
        if history.contains(&(
            self.players[0].deck.iter().copied().collect(),
            self.players[1].deck.iter().copied().collect(),
        )) {
            return GameResult::GameOver(0);
        }
        history.push((
            self.players[0].deck.iter().copied().collect(),
            self.players[1].deck.iter().copied().collect(),
        ));
        let card1 = self.players[0].deck.pop_front().unwrap();
        let card2 = self.players[1].deck.pop_front().unwrap();
        let winner = if card1 as usize <= self.players[0].deck.len()
            && card2 as usize <= self.players[1].deck.len()
        {
            let mut sub_game = SpaceCards {
                players: [
                    Player::new(
                        "Player 1",
                        &self.players[0].deck.as_slices().0[..card1 as usize],
                    ),
                    Player::new(
                        "Player 2",
                        &self.players[1].deck.as_slices().0[..card2 as usize],
                    ),
                ],
            };
            sub_game.part2_game()
        } else if card1 > card2 {
            0
        } else {
            1
        };
        if winner == 0 {
            self.players[0].deck.push_back(card1);
            self.players[0].deck.push_back(card2);
        } else {
            self.players[1].deck.push_back(card2);
            self.players[1].deck.push_back(card1);
        }
        GameResult::OneRound(winner)
    }

    pub fn part2_game(&mut self) -> usize {
        let mut history = vec![];
        loop {
            match self.part2_next_round(&mut history) {
                GameResult::GameOver(winner) => return winner,
                GameResult::OneRound(winner) => {
                    if self.players.iter().any(|p| p.deck.is_empty()) {
                        return winner;
                    }
                }
            }
        }
    }
}

pub fn read_input(input_file: &str) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

pub fn all_solution<T>(cards: &mut SpaceCards, game_fn: T) -> u32
where
    T: FnOnce(&mut SpaceCards) -> usize,
{
    let winner = game_fn(cards);
    cards.players[winner]
        .deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &c)| (i as u32 + 1) * c)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut testcase = SpaceCards::new(&read_input("../testcase1.txt"));
        assert_eq!(all_solution(&mut testcase, SpaceCards::part1_game), 306);
    }

    #[test]
    fn test_part2() {
        let mut testcase = SpaceCards::new(&read_input("../testcase1.txt"));
        assert_eq!(all_solution(&mut testcase, SpaceCards::part2_game), 291);
    }
}
