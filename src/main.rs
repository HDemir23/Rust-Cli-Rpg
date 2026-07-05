pub mod battle;
pub mod enemy_gen;
pub mod player_progression;
mod read_input;
pub mod structs;

use crate::enemy_gen::gen_enemy;
use battle::battle;
use read_input::read_input;
use structs::{Enemy, Player};

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
        damage: 20,
        gold: 0,
        level: 1,
        xp: 0,
        xp_to_next_level: 35,
    }
}

fn run_game_loop(player: &mut Player) {
    loop {
        let mut enemy = gen_enemy(player);

        print_enemy_intro(&enemy);
        battle(player, &mut enemy);

        if player_is_dead(player) {
            print_game_over(player);
            break;
        }

        if !should_continue_playing() {
            break;
        }
    }
}

fn print_enemy_intro(enemy: &Enemy) {
    println!("\nA wild {} appeared!", enemy.name);
    println!("Level: {}", enemy.level);
    println!("Rarity: {:?}", enemy.rarity);
    println!("Health: {}", enemy.health);
    println!("Damage: {}", enemy.damage);
}

fn player_is_dead(player: &Player) -> bool {
    player.health <= 0
}

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
