#[allow(dead_code)]
#[allow(unused_variables)]
use cardpack::{Card, Pile, Rank, Standard52};
use std::collections::HashMap;
use rust_go_fish::{get_random_excluding, get_random};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PlayerTurnResult {
    ReceiveCard(Card),
    DrawFromDeck,
    EndGame
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
        println!("self.hand.ranks(): {:?}", self.hand.ranks());
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
}

impl GameState {
    fn new(player_count: usize) -> Self {
        let deck = Standard52::new_shuffled();
        // create players
        let players = (0..player_count).map(|_| Player::new()).collect();
        GameState { deck, players, player_count  }
    }

    fn new_with_deck(player_count: usize, deck: Standard52) -> Self {
        // create players
        let players = (0..player_count).map(|_| Player::new()).collect();
        GameState { deck, players, player_count  }
    }

    fn deal(&mut self) {
        // let hand_size = 52 / self.player_count;
        let hand_size = 7;
        for _ in 0..hand_size {
            for player in &mut self.players {
                player.add_cards(self.deck.draw(1).unwrap());
            }
        }
    }

    fn players_match_cards(&mut self) {
        self.players.iter_mut().for_each(|player| {
            let _pairs_found = player.match_pairs();
            // println!("pairs: {:?}", &player.pairs.to_symbol_index());
        });
    }

    fn play_turn(&mut self, player_index: usize) -> PlayerTurnResult {
        // get random card from player's hand and select another player to ask
        let random_card_index = get_random(0..self.players[player_index].hand.len());
        let other_player_index = get_random_excluding(0..self.player_count, player_index);
        self.play_turn_with_indexes_card(player_index, other_player_index, random_card_index)
    }

    fn play_turn_with_indexes_card(&mut self, player_index: usize,  other_player_index: usize, card_index: usize) -> PlayerTurnResult{
        // Logic for a player's turn

        // get random card from player's hand
        let card = self.players[player_index].hand.cards()[card_index].clone();
        self.ask_for_card(player_index, other_player_index, card)
    }

    fn check_winner(&self) -> Option<usize> {
        // Check if there's a winner
        None
    }

    /// Transfer cards from one player to another
    fn transfer_cards(&mut self, player_index: usize, other_player_index: usize, card: Card) {
        // Logic for transferring cards
        self.players[other_player_index].hand.remove_card(&card);
        self.players[player_index].hand.push(card);
    }

    // Ask another player for a card
    // If they have it, take it and ask again
    // If they don't, draw a card from the deck
    // If the deck is empty, end the game
    fn ask_for_card(&mut self, player_index: usize, answering_player_index: usize, card: Card) -> PlayerTurnResult {
        // Logic for asking for a card
        // pick another player
        // let other_player_index = get_random_excluding(0..self.player_count, player_index);
        // let other_player = &mut self.players[other_player_index];
        let result =  self.players[answering_player_index].answer_for_card_rank(card.rank);
        // check if they have the card
        match result {
        // match other_player.answer_for_card_rank(card.rank) {
            Some(card) => {
                // take the card
                self.transfer_cards(player_index, answering_player_index, card.clone());
                // other_player.hand.remove_card(&card);
                // let player = &mut self.players[player_index];
                //
                // player.hand.push(card);
                // self.players[player_index].match_pairs();

                // // get random card from player's hand
                // let random_card = get_random(0..self.players[player_index].hand.len());
                // let card = self.players[player_index].hand.cards()[random_card].clone();
                // // // ask again
                // let next_player_index = get_random_excluding(0..self.player_count, player_index);
                // self.ask_for_card(player_index, next_player_index, card);
                PlayerTurnResult::ReceiveCard(card)

            },
            None => {
                // draw a card from the deck
                match self.deck.draw(1) {
                    Some(card) => {
                        // add the card to the player's hand
                        let player = &mut self.players[player_index];

                        player.hand.append(&card);
                        PlayerTurnResult::DrawFromDeck
                    },
                    None => {
                        // end the game
                        println!("Game over!");
                        PlayerTurnResult::EndGame
                    }
                }
            }
        }
    }
}

fn main() {
    let mut game = GameState::new(4);

    game.deal();


    // for player in &game.players {
    //     println!("player: {:?}", &player.hand.to_symbol_index());

        // let pairs_found = player.match_pairs();
    // }

    // find pairs
    game.players.iter_mut().for_each(| player| {
        println!("player Hand: {:?}", &player.hand.to_symbol_index());
        let pairs_found = player.match_pairs();
        println!("pairs_found: {:?}", pairs_found);
        println!("player hand: {:?}", &player.hand.to_symbol_index());
        println!("pairs: {:?}", &player.pairs.to_symbol_index());
    });





    // let pack = cardpack::Standard52::default();
    let mut pack = Standard52::new_shuffled();
    println!("count : {}", pack.deck.len());
    // // println!("pack: {:?}", pack);
    // println!("turn : {}", pack.deck.len());
    let player1_cards = pack.draw(2).unwrap();
    println!("player1_cards : {}", player1_cards.to_symbol_index());
    let p3 = player1_cards.clone();
    for card in player1_cards.cards() {
        println!("card : {}", card.index);
        let c = p3.position(card).unwrap();
        println!("c : {}", c);
    }
    // println!("player1_cards : {:?}", player1_cards);
    // loop {
    //     // Game loop
    // }
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
        // let index_string = "2S 3D QS KH 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";
        // let standard52 = Standard52::from_index(index_string).unwrap();
        let standard52 = Standard52::default();
        let mut game = GameState::new_with_deck(4, standard52);
        game.deal();
        game.players_match_cards();
        game
    }

    // #[test]
    // fn test_base_case() {
    //     let input_data = "TestData";
    //     // let input_data = fs::read_to_string("data/ex1.ex.txt").expect("Unable to read file");
    //     // println!("input_data: {}", input_data);
    //
    //     let result = exercise_p1(input_data);
    //     assert_eq!(result, 100);
    // }

    #[test]
    fn test_find_pair() {
        // let index_string = "2S 3D QS KH 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";
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

        println!("result: {:?}", result);
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
        for player in &game.players {
            println!("player: {:?}", &player.hand.to_index_str());
        }
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.players[0].hand.len(), 7);

        assert_eq!(game.players[0].hand, Standard52::pile_from_index("AS TS 6S 2S JH 7H 3H").unwrap());
        assert_eq!(game.players[1].hand, Standard52::pile_from_index("KS 9S 5S AH TH 6H 2H").unwrap());
        assert_eq!(game.players[2].hand, Standard52::pile_from_index("QS 8S 4S KH 9H 5H AD").unwrap());
        assert_eq!(game.players[3].hand, Standard52::pile_from_index("JS 7S 3S QH 8H 4H KD").unwrap());
    }

    #[test]
    fn test_player_ask_for_card() {
        let mut game  = setup();
        let card = Standard52::card_from_index("6S");
        let current_player_index = 0;
        let answering_player_index = 1;
        let player_turn_result = game.ask_for_card(current_player_index, answering_player_index, card);
        println!("player_turn_result: {:?}", player_turn_result);
        // assert_eq!(player_turn_result, PlayerTurnResult::ReceiveCard(Standard52::card_from_index("6H")));
        assert_eq!(game.players[current_player_index].hand.len(), 8);
        game.players[current_player_index].match_pairs();
        assert_eq!(game.players[current_player_index].pairs.len(), 2);
    }
}
