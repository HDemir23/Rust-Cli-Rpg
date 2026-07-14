use crate::game_state::App;
use crate::structs::{Dungeon, Player};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    player: Player,
    dungeon: Dungeon,
    dungeon_level: u32,
}

pub fn save_game(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let data = SaveData {
        player: app.player.clone(), // or move/copy the fields appropriately
        dungeon_level: app.dungeon_level,
        dungeon: app.dungeon.clone(),
    };

    let json = serde_json::to_string_pretty(&data)?;
    std::fs::write("save.json", json)?;

    Ok(())
}

pub fn load_player() -> Option<Player> {
    let json = std::fs::read_to_string("save.json").ok()?;
    let data: SaveData = serde_json::from_str(&json).ok()?;

    Some(data.player)
}
