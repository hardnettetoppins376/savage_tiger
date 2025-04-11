use std::collections::HashMap;

struct Data {
    players: HashMap<String, String>,
}

impl Data {
    fn new(players: Vec<String>) -> Self {
        Self { players }
    }

    fn add_player(&mut self, player: &str) {
        if let Some(existing_player) = self.players.get(player) {
            eprintln!("Player {} is already registered.", existing_player);
        } else {
            self.players.insert(player.clone(), String::from("player_{}", self.player_count().unwrap()));
        }
    }

    fn remove_player(&mut self, player: &str) {
        if let Some(existing_player) = self.players.get(player) {
            self.players.remove(existing_player);
            eprintln!("Player {} removed.", existing_player);
        } else {
            eprintln!("Player {} not found in the list.", player);
        }
    }

    fn print_players(&self) {
        println!(
            "Players: {:?}",
            players
                .iter()
                .map(|player| String::from(player))
                .collect::<Vec<_>>()
        );
    }
}

fn main() {
    let data = Data::new(vec!["user1", "user2", "user3"]);
    println!("Data: {:?}", data);
}
