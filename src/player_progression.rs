use crate::structs::Player;

pub fn player_health_increase(player: &Player) -> i32 {
    player.health_amount + (player.max_health * 5) / 100
}

/// Handles player level-up rewards while enough XP is available.
pub fn check_level_up(player: &mut Player) {
    while player.xp >= player.xp_to_next_level {
        player.xp -= player.xp_to_next_level;
        player.level += 1;

        player.max_health += 20;
        player.health = player.max_health;
        player.damage += 5;
        player.xp_to_next_level += player.level * 50;
        player.health_amount += player_health_increase(&player);

        println!("LEVEL UP!");
        println!("You are now level {}", player.level);
        println!("Max health increased to {}", player.max_health);
        println!("Damage increased to {}", player.damage);
    }
}
