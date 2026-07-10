use crate::structs::Player;

pub fn player_health_increase(player: &Player) -> i32 {
    player.health_amount + (player.max_health * 5) / 100
}

/// Handles player level-up rewards while enough XP is available.
pub fn check_level_up(player: &mut Player) -> Vec<String> {
    let mut messages = Vec::new();

    while player.xp >= player.xp_to_next_level {
        player.xp -= player.xp_to_next_level;
        player.level += 1;
        player.max_health += 20;
        player.health = player.max_health;
        player.damage += 5;
        player.xp_to_next_level += player.level * 45;

        messages.push("LEVEL UP!".to_string());
        messages.push(format!("You are now level {}", player.level));
        messages.push(format!("Max health increased to {}", player.max_health));
        messages.push(format!("Damage increased to {}", player.damage));
    }

    messages
}
