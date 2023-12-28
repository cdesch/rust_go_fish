#[allow(dead_code)]
#[allow(unused_variables)]
use cardpack::{Card, Pile, Rank, Standard52};
use std::collections::HashMap;
use rust_go_fish::{get_random_excluding, get_random};

enum Environment {
    Development,
    Test,
    Production,
}

enum GameMode {
    Random,
    Sequential,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EndGameConditions {
    FirstPlayerWithNoCards,
    DeckEmpty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EndGameCondition {
    Winner(usize),
    Tie(Vec<usize>),
    Continue
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PlayerTurnResult {
    NextPlayerTurn,
    PlayAgain,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum AskCardResult {
    ReceiveCard(Card),
    GoFish
}




fn find_pair(pile: &Pile) -> Option<(Card, Card)> {

    let mut rank_count = HashMap::new();

    for card in pile.cards() {
        let count = rank_count.entry(card.rank).or_insert(0);
        *count += 1;

        if *count == 2 {
            // Find the first card that matches this rank
            let first_card = pile.cards().iter().find(|&c| c.rank == card.rank).unwrap();

            return Some((first_card.clone(), card.clone()));
        }
    }

    None
}

struct Player {
    hand: Pile,
    pairs: Pile,
    score: u32,
}

impl Player {
    fn new() -> Self {
        Player {
            hand: Pile::default(),
            pairs: Pile::default(),
            score: 0,
        }
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn add_cards(&mut self, cards: Pile) {
        self.hand.append(&cards);
    }

    fn answer_for_card_rank(&self, rank: Rank) -> Option<Card> {
        // Check if the player has a card of the given rank
        // If so, return it
        // If not, return None
        // println!("self.hand.ranks(): {:?}", self.hand.ranks());
        self.hand.ranks().iter().find(|&r| r == &rank).map(|r| self.hand.cards().iter().find(|&c| c.rank == *r).unwrap().clone())
        // self.hand.cards().iter().find(|&c| c.rank == rank).map(|c| c.clone())
    }

    fn match_pairs(&mut self) -> u32 {
        // Check for pairs
        // If pair, move to pairs pile and add to the score
        self.hand = self.hand.sort_by_frequency();
        let mut pairs_found_count = 0;
        while let Some((card1, card2)) = find_pair(&self.hand) {
            self.hand.remove_card(&card1);
            self.hand.remove_card(&card2);
            self.pairs.push(card1);
            self.pairs.push(card2);
            self.score += 1;
            pairs_found_count += 1;
        }
        pairs_found_count

    }
}

struct GameState {
    deck: Standard52,
    players: Vec<Player>,
    player_count: usize,
    game_mode: GameMode,
}

impl GameState {
    /// Create a new game with a shuffled deck
    fn new(player_count: usize) -> Self {
        let deck = Standard52::new_shuffled();
        // create players
        let players = (0..player_count).map(|_| Player::new()).collect();
        GameState { deck, players, player_count, game_mode: GameMode::Random}
    }

    /// Create a new game with a specific deck
    fn new_with_deck(player_count: usize, deck: Standard52, game_mode: GameMode) -> Self {
        // create players
        let players = (0..player_count).map(|_| Player::new()).collect();
        GameState { deck, players, player_count, game_mode }
    }

    /// Deal cards to each player
    fn deal(&mut self) {
        // let hand_size = 52 / self.player_count;
        let hand_size = 7;
        for _ in 0..hand_size {
            for player in &mut self.players {
                player.add_cards(self.deck.draw(1).unwrap());
            }
        }
    }

    /// Match cards in each player's hand
    fn players_match_cards(&mut self) {
        self.players.iter_mut().for_each(|player| {
            let _pairs_found = player.match_pairs();
        });
    }

    /// Get the index of the next player
    /// If the player is the last player, return the first player
    /// Otherwise, return the next player
    ///
    /// # Arguments
    ///
    /// * `player_index` - The index of the current player
    ///
    /// # Example
    ///
    /// ```
    /// use rust_go_fish::GameState;
    ///
    /// let game = GameState::new(4);
    /// let next_player_index = game.next_player_index(0);
    /// assert_eq!(next_player_index, 1);
    /// ```
    fn next_player_index(&self, player_index: usize) -> usize {
        (player_index + 1) % self.player_count
    }

    fn play_turn(&mut self, player_index: usize) -> PlayerTurnResult {
        println!("player {} turn", player_index);
        // get random card from player's hand and select another player to ask
        let (random_card_index, other_player_index) = self.get_turn_indices(player_index);
        self.perform_turn(player_index, other_player_index, random_card_index)
    }

    fn get_turn_indices(&self, player_index: usize) -> (usize, usize) {
        let card_index = 0;
        match self.game_mode {
            GameMode::Random => {
                let random_card_index = get_random(0..self.players[player_index].hand.len());
                let other_player_index = get_random_excluding(0..self.player_count, player_index);
                (random_card_index, other_player_index)
            },
            GameMode::Sequential => {
                let random_card_index = 0;
                let other_player_index = self.next_player_index(player_index);
                (random_card_index, other_player_index)
            }
        }
    }

    fn perform_turn(&mut self, player_index: usize, other_player_index: usize, card_index: usize) -> PlayerTurnResult{
        // Logic for a player's turn
        // get random card from player's hand
        let card = self.players[player_index].hand.cards()[card_index].clone();
        match self.ask_for_card(player_index, other_player_index, card) {
            AskCardResult::ReceiveCard(card) => {
                println!("player {} received card: {}", player_index, card);
                self.players[player_index].match_pairs();
                PlayerTurnResult::PlayAgain
            },
            AskCardResult::GoFish => {
                println!("player {} go fish", player_index);
                match self.deck.draw(1) {
                    Some(card) => {
                        println!("player {} drew from deck", player_index);
                        let player = &mut self.players[player_index];

                        // add the card to the player's hand
                        player.hand.append(&card);
                        PlayerTurnResult::NextPlayerTurn
                    },
                    None => {
                        PlayerTurnResult::NextPlayerTurn
                    }
                }
            }
        }
    }

    /// Check if the game should end
    fn check_win_condition(&self) -> EndGameCondition {

        // find any player with no cards in their hand
        let empty_hands_index = self
            .players
            .iter()
            .enumerate()
            .find(|&(_, player)|
                player.hand.len() == 0)
            .map(|(index, _)| index);

        // check to see if anyone no longer has cards in their hands
        match empty_hands_index {
            Some(index) => {
                // determine a winner if any player has no more cards
                println!("player {} has no more cards", index);
                self.determine_winner()
            }
            None => {
                // continue play
                EndGameCondition::Continue
            }
        }
    }

    /// Determine the winner of the game
    fn determine_winner(&self) -> EndGameCondition {
        let winners = self.indices_of_max_score_players();
        match winners.len() {
            0 => EndGameCondition::Continue,
            1 => EndGameCondition::Winner(winners[0]),
            _ => EndGameCondition::Tie(winners)
        }
    }

    /// Get the indices of the players with the highest score
    fn indices_of_max_score_players(&self) -> Vec<usize> {
        if self.players.is_empty() {
            return Vec::new();
        }

        let max_score = self.players.iter()
            .map(|player| player.score)
            .max()
            .unwrap(); // Safe to use unwrap as players is not empty

        self.players.iter()
            .enumerate()
            .filter_map(|(index, player)| {
                if player.score == max_score {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Transfer cards from one player to another
    fn transfer_cards(&mut self, player_index: usize, other_player_index: usize, card: Card) {
        // Logic for transferring cards
        self.players[other_player_index].hand.remove_card(&card);
        self.players[player_index].hand.push(card);
    }

    /// Ask another player for a card
    /// If they have it, take it
    /// If they don't, GoFish
    fn ask_for_card(&mut self, player_index: usize, answering_player_index: usize, card: Card) -> AskCardResult {
        let result =  self.players[answering_player_index].answer_for_card_rank(card.rank);
        // check if they have the card
        match result {
            Some(card) => {
                // take the card
                self.transfer_cards(player_index, answering_player_index, card.clone());
                AskCardResult::ReceiveCard(card)
            },
            None => {
                AskCardResult::GoFish
            }
        }
    }
}

fn main() {

    // Test
    // let standard52 = Standard52::default();
    // let mut game = GameState::new_with_deck(4, standard52, GameMode::Sequential);
    // game.deal();
    // game.players_match_cards();
    // let _game_result  = run_game(&mut game);

    // Run

    let mut game = GameState::new(4);
    game.deal();
    game.players_match_cards();
    let _game_result  = run_game(&mut game);

}

fn run_game(game: &mut GameState) -> EndGameCondition {

    let mut current_player_index = 0;
    let mut end_game_condition = EndGameCondition::Continue;

    // Game Loop
    while end_game_condition == EndGameCondition::Continue {
        let mut turn_result = PlayerTurnResult::PlayAgain;
        while turn_result == PlayerTurnResult::PlayAgain {
            // Update the outer 'turn_result' variable with the new turn result
            turn_result = game.play_turn(current_player_index);
            // println!("player_turn_result: {:?}", turn_result);

            // Check the end game condition after each turn
            end_game_condition = game.check_win_condition();
            if end_game_condition != EndGameCondition::Continue {
                break; // Break from the inner loop if the game should end
            }
        }
        current_player_index = game.next_player_index(current_player_index);
        // println!("current_player_index: {:?}", current_player_index);
    }

    // println!("end_game_condition: {:?}", end_game_condition);
    handle_end_game_condition(&end_game_condition, &game);
    end_game_condition
}

fn handle_end_game_condition(condition: &EndGameCondition, game_state: &GameState) {
    match condition {
        EndGameCondition::Winner(winner_index) => {
            print_winner_info(winner_index, game_state);
        },
        EndGameCondition::Tie(tie_indices) => {
            println!("There is a tie between players at indices {:?}", tie_indices);
            for index in tie_indices {
                print_winner_info(index, game_state);
                println!("");
            }
        },
        EndGameCondition::Continue => {
            // println!("The game continues");
        },
    }
}

fn print_winner_info(winner_index: &usize, game_state: &GameState) {
    println!("The winner is player at index {}", winner_index);
    println!("With a score of {}", game_state.players[*winner_index].score);
    println!("With the following matching Pairs {}", game_state.players[*winner_index].pairs.to_index());
}

#[cfg(test)]
mod tests {
    use cardpack::{JACK, TWO};
    use super::*;

    fn setup_random() -> GameState {
        let mut game = GameState::new(4);
        game.deal();
        game.players_match_cards();
        game
    }

    fn setup() -> GameState {
        let standard52 = Standard52::default();
        let mut game = GameState::new_with_deck(4, standard52, GameMode::Sequential);
        game.deal();
        game.players_match_cards();
        game
    }

    #[test]
    fn test_find_pair() {
        let index_string = "2S 2D QS KH 3C 3S";
        let pile = Standard52::pile_from_index(index_string).unwrap();
        let result = find_pair(&pile).unwrap();

        assert_eq!(result, ( Standard52::card_from_index("2S"), Standard52::card_from_index("2D")));

    }

    #[test]
    fn test_player_answer_for_card_rank_in_hand() {
        let index_string = "2S 2D QS KH 3C 3S";
        let pile = Standard52::pile_from_index(index_string).unwrap();
        let mut player = Player::new();
        player.add_cards(pile);
        let result = player.answer_for_card_rank(Rank::new(TWO)).unwrap();

        // println!("result: {:?}", result);
        assert_eq!(result, Standard52::card_from_index("2S"));
    }

    #[test]
    fn test_player_answer_for_card_rank_not_in_hand() {
        let index_string = "2S 2D QS KH 3C 3S";
        let pile = Standard52::pile_from_index(index_string).unwrap();
        let mut player = Player::new();
        player.add_cards(pile);
        let result = player.answer_for_card_rank(Rank::new(JACK));
        assert_eq!(result, None);
    }

    #[test]
    fn test_player_match_pairs() {
        let index_string = "2S 2D QS KH 3C 3S";
        let pile = Standard52::pile_from_index(index_string).unwrap();
        let mut player = Player::new();
        player.add_cards(pile);
        let result = player.match_pairs();
        assert_eq!(result, 2);
        assert_eq!(player.hand.len(), 2);
        assert_eq!(player.pairs.len(), 4);


    }

    #[test]
    fn test_setup_game(){
        let game = setup();
        // for player in &game.players {
        //     println!("player: {:?}", &player.hand.to_index_str());
        // }
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.players[0].hand.len(), 7);

        assert_eq!(game.players[0].hand.to_index_str(), "AS JH TS 7H 6S 3H 2S");
        assert_eq!(game.players[1].hand.to_index_str(), "AH KS TH 9S 6H 5S 2H");
        assert_eq!(game.players[2].hand.to_index_str(), "AD KH QS 9H 8S 5H 4S");
        assert_eq!(game.players[3].hand.to_index_str(), "KD QH JS 8H 7S 4H 3S");
    }

    #[test]
    fn test_player_ask_for_card() {
        let mut game  = setup();
        let card = Standard52::card_from_index("6S");
        let current_player_index = 0;
        let answering_player_index = 1;
        let player_turn_result = game.ask_for_card(current_player_index, answering_player_index, card);
        // println!("player_turn_result: {:?}", player_turn_result);
        // assert_eq!(player_turn_result, PlayerTurnResult::ReceiveCard(Standard52::card_from_index("6H")));
        assert_eq!(game.players[current_player_index].hand.len(), 8);
        game.players[current_player_index].match_pairs();
        assert_eq!(game.players[current_player_index].pairs.len(), 2);
    }

    #[test]
    fn test_game_play_turn() {
        let mut game  = setup();
        let card = Standard52::card_from_index("6S");
        let current_player_index = 0;
        let answering_player_index = 1;
        let player_turn_result = game.play_turn(current_player_index);
        // println!("player_turn_result: {:?}", player_turn_result);

        assert_eq!(game.players[current_player_index].hand.len(), 6);
        game.players[current_player_index].match_pairs();
        assert_eq!(game.players[current_player_index].pairs.len(), 2);
    }

    #[test]
    fn test_run_game() {
        let mut game  = setup();
        let result = run_game(&mut game);
        match result {
            EndGameCondition::Winner(winner_index) => {
                assert_eq!(winner_index, 2);
                assert_eq!(game.players[winner_index].score, 8);
                assert_eq!(game.players[winner_index].pairs.to_index_str(), "AD AC QS QH 8S 8H 5H 5D 4S 4H 2D 2C JS JC 3S 3C");

            },
            EndGameCondition::Tie(tie_indices) => {
            },
            EndGameCondition::Continue => {}
        }
        // assert_eq!(game.deck.deck.len(), 0);
    }

    #[test]
    fn test_game_next_player_index(){
        let game = setup();
        let next_player_index = game.next_player_index(0);
        assert_eq!(next_player_index, 1);
    }

    // #[test]
    // fn test_game_indices_of_max_score_players(){
    //     let mut game = setup();
    //     game.players[0].score = 1;
    //     game.players[1].score = 2;
    //     game.players[2].score = 3;
    //     game.players[3].score = 4;
    //     let winner_index = game.indices_of_max_score_players().unwrap();
    //     assert_eq!(max_score_index, 3);
    //
    //     let mut game = setup();
    //     game.players[0].score = 1;
    //     game.players[1].score = 4;
    //     game.players[2].score = 4;
    //     game.players[3].score = 3;
    //     let max_score_index = game.indices_of_max_score_players().unwrap();
    //     assert_eq!(max_score_index, 3);
    // }
}
