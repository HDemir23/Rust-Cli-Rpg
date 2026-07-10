use crate::player_progression::{check_level_up, player_health_increase};
use crate::structs::{BattleOutcome, BattleTurnResult, Enemy, Player, PlayerAction, TurnResult};

fn player_action(action: PlayerAction, player: &mut Player, enemy: &mut Enemy) -> TurnResult {
    match action {
        PlayerAction::Attack => {
            player_attack(player, enemy);
            TurnResult::Continue
        }
        PlayerAction::Heal => {
            heal_player(player);
            TurnResult::Continue
        }
        PlayerAction::Run => TurnResult::EndBattle(BattleOutcome::Fled),
        PlayerAction::Quit => TurnResult::EndBattle(BattleOutcome::Quit),
    }
}

fn player_attack(player: &Player, enemy: &mut Enemy) {
    enemy.health -= player.damage;
}

fn heal_player(player: &mut Player) {
    let heal_amount = player_health_increase(player);
    player.health = (player.health + heal_amount).min(player.max_health);
}

fn enemy_is_defeated(enemy: &Enemy) -> bool {
    enemy.health <= 0
}

fn reward_player(player: &mut Player, enemy: &Enemy) -> Vec<String> {
    player.gold += enemy.gold_reward;
    player.xp += enemy.xp_reward;
    check_level_up(player)
}

fn enemy_attack(player: &mut Player, enemy: &Enemy) {
    player.health -= enemy.damage;
}

fn player_is_defeated(player: &Player) -> bool {
    player.health <= 0
}

pub fn battle_turn(
    player: &mut Player,
    enemy: &mut Enemy,
    action: PlayerAction,
) -> BattleTurnResult {
    let mut messages = Vec::new();

    match player_action(action, player, enemy) {
        TurnResult::EndBattle(outcome) => {
            return BattleTurnResult {
                outcome: Some(outcome),
                messages,
            };
        }
        TurnResult::Continue => {}
    }

    if enemy_is_defeated(enemy) {
        messages.extend(reward_player(player, enemy));
        return BattleTurnResult {
            outcome: Some(BattleOutcome::Victory),
            messages,
        };
    }

    enemy_attack(player, enemy);

    let outcome = player_is_defeated(player).then_some(BattleOutcome::Defeat);
    BattleTurnResult { outcome, messages }
}
