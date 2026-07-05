pub mod structs;
mod read_input;
pub mod battle;
pub mod enemy_gen;


use battle::battle;
use read_input::read_input;
use structs::{Player};
use crate::enemy_gen::gen_enemy;

fn main() {
    let mut player = Player {
        name: read_input("Enter Ur name"),
        health: 100,
        damage: 20,
        gold: 0,
        level: 1,
    };

    loop {
        let mut enemy = gen_enemy(&player);


        println!("\nA wild {} appeared!", enemy.name);
        println!("Level: {}", enemy.level);
        println!("Rarity: {:?}", enemy.rarity);
        println!("Health: {}", enemy.health);
        println!("Damage: {}", enemy.damage);

        battle(&mut player, &mut  enemy);

        if player.health <= 0 {
            println!("Game Over");
            break;
        }

        let command = read_input("continue playing y / n");

        match  command.as_str() {
            "y" => { continue },
            "n" => {
                println!("Exiting From Dungeon");
                break;
            },
            _ => {
                println!("Invalid command");
                break;},
            }


    }
}














