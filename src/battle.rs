use crate::player_progression::{check_level_up, player_health_increase};
use crate::read_input::read_input;
use crate::structs::{Enemy, Player, TurnReslut};

/// Runs one battle encounter between the player and the active enemy.
pub fn battle(player: &mut Player, enemy: &mut Enemy) {
    // Continue until the player, enemy, or command flow ends the encounter.
    loop {
        print_enemy_status(enemy);

        // Read the player's turn command.
        let command = read_input("command: attack => a / heal => e / run => r / quit q");

        // Apply the selected player action.
        match player_action(&command, player, enemy) {
            TurnReslut::SkipEnemyTurn => continue,
            TurnReslut::EndBattle => break,
            TurnReslut::Continue => {}
        }

        // Reward the player when the enemy is defeated.
        if enemy_is_defeated(enemy) {
            reward_player(player, enemy);
            break;
        }

        enemy_attack(player, enemy);

        // End the encounter when the player is defeated.
        if player_is_defeated(player) {
            break;
        }
    }
}

fn print_enemy_status(enemy: &Enemy) {
    println!(
        "Enemy: {} Health: {} , Damage: {}",
        enemy.name, enemy.health, enemy.damage
    );
}

fn player_action(command: &str, player: &mut Player, enemy: &mut Enemy) -> TurnReslut {
    match command {
        "a" => {
            player_attack(player, enemy);
            TurnReslut::Continue
        }
        "e" => {
            heal_player(player);
            TurnReslut::Continue
        }
        "r" => {
            println!("you run away");
            TurnReslut::EndBattle
        }
        "q" => TurnReslut::EndBattle,
        _ => {
            println!("Unknown command: {}", command);
            TurnReslut::SkipEnemyTurn
        }
    }
}

fn player_attack(player: &Player, enemy: &mut Enemy) {
    enemy.health -= player.damage;
    println!("You attacked {} for {} damage", enemy.name, player.damage);
}

fn heal_player(player: &mut Player) {
    let heal_amount = player_health_increase(player);

    player.health += heal_amount;

    if player.health > player.max_health {
        player.health = player.max_health;
    }

    println!("Healed {} health: {}", heal_amount, player.health);
}

fn enemy_is_defeated(enemy: &Enemy) -> bool {
    enemy.health <= 0
}

fn reward_player(player: &mut Player, enemy: &Enemy) {
    player.gold += enemy.gold_reward;
    player.xp += enemy.xp_reward;

    println!("You defeated {}", enemy.name);
    println!("Gold gained: {}", enemy.gold_reward);
    println!("XP gained: {}", enemy.xp_reward);

    check_level_up(player);
}

fn enemy_attack(player: &mut Player, enemy: &Enemy) {
    player.health -= enemy.damage;
    println!("Ouch! health: {}", player.health);
}

fn player_is_defeated(player: &Player) -> bool {
    player.health <= 0
}
