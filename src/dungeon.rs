use crate::battle::battle;
use crate::enemy_gen::gen_enemy;
use crate::read_input::read_input;
use crate::structs::{Dungeon, Enemy, Player};

pub fn run_dungeon(player: &mut Player, dungeon_level: u32) -> bool {
    let mut dungeon = Dungeon::new(dungeon_level);

    println!("Entering Dungeon level {}", dungeon.level);

    while !dungeon.is_cleared() {
        println!("Dungeon Level {} - Room {}", dungeon.level, dungeon.current_room);

        let mut enemy = gen_enemy(player, dungeon_level, dungeon.is_boss_room());

        print_enemy_intro(&enemy);

        battle(player, &mut enemy);

        if player.health <= 0 {
            return false;
        }
        dungeon.advance_room();
    }

    reward_dungeon_clear(player, dungeon_level);
    true
}

fn print_enemy_intro(enemy: &Enemy) {
    println!("\nA wild {} appeared! \n Level: {} ", enemy.name, enemy.level);
    println!("Rarity: {:?}", enemy.rarity);
    println!("Health: {}", enemy.health);
    println!("Damage: {}", enemy.damage);
}

fn reward_dungeon_clear(player: &mut Player, dungeon_level: u32) {
    let gold_bonus = dungeon_level * 25;
    let xp_bonus = dungeon_level * 20;


    player.gold += gold_bonus;
    player.xp += xp_bonus;

    println!("Dungeon Clerde");
    println!("Gold {} xp: {}", player.gold, player.xp);
}

pub fn should_enter_next_dungeon() -> bool {
    let command = read_input("Enter the Next Dungeon y / n");
    match command.as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("invalid command");
            false
        }
    }
}
