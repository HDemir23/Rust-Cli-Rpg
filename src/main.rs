pub mod battle;
pub mod enemy_gen;
pub mod player_progression;
mod read_input;
pub mod structs;
pub mod dungeon;

use crate::dungeon::run_dungeon;
use crate::structs::DungeonOutcome;
use read_input::read_input;
use structs::Player;

fn main() {
    let mut player = create_player();

    run_game_loop(&mut player);
}

fn create_player() -> Player {
    Player {
        name: read_input("Enter Ur name"),
        health: 100,
        health_amount: 25,
        max_health: 100,
        damage: 25,
        gold: 0,
        level: 1,
        xp: 0,
        xp_to_next_level: 35,
    }
}

fn run_game_loop(player: &mut Player) {
    let mut dungeon_level = 1;

    loop {
        let dungeon_res = run_dungeon(player, dungeon_level);

        match dungeon_res {
            DungeonOutcome::Cleared => {
                dungeon_level += 1;

                if !should_continue_playing() {
                    println!("Exiting Dungeon");
                    break;
                }
            }
            DungeonOutcome::PlayerDied => {
                print_game_over(player);
                break;
            }
            DungeonOutcome::Left => {
                println!("Left dungeon");
                break;
            }
        }
    }
}


// fn player_is_dead(player: &Player) -> bool {
//     player.health <= 0
// }

fn print_game_over(player: &Player) {
    println!("Game Over");
    println!("Gained Gold: {}", player.gold);
    println!("Gained Xp: {}", player.xp);
}

fn should_continue_playing() -> bool {
    let command = read_input("continue playing y / n");

    match command.as_str() {
        "y" => true,
        "n" => {
            println!("Exiting From Dungeon");
            false
        }
        _ => {
            println!("Invalid command");
            false
        }
    }
}
