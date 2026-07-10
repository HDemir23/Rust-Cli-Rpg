use crate::battle::battle_turn;
use crate::enemy_gen::gen_enemy;
use crate::player_progression::check_level_up;
use crate::structs::{BattleOutcome, Dungeon, Enemy, Player, PlayerAction};
pub struct App {
    pub player: Player,
    pub dungeon_level: u32,
    pub dungeon: Dungeon,
    pub enemy: Option<Enemy>,
    pub message: Vec<String>,
    pub should_quit: bool,
    pub dungeon_finished: bool,
}

impl App {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            dungeon_level: 1,
            dungeon: Dungeon::new(1),
            enemy: None,
            message: Vec::new(),
            should_quit: false,
            dungeon_finished: false,
        }
    }

    pub fn spawn_next_enemy(&mut self) {
        if self.enemy.is_some() {
            return;
        }

        if self.dungeon.is_cleared() {
            if !self.dungeon_finished {
                self.message.push("Dungeon cleared".to_string());
                self.dungeon_finished = true;
            }
            return;
        }

        let enemy = gen_enemy(
            &self.player,
            self.dungeon_level,
            self.dungeon.is_boss_room(),
        );

        self.message.push(format!("A wild {} appeared!", enemy.name));
        self.enemy = Some(enemy);
    }

    pub fn handle_battle_action(&mut self, action: PlayerAction) {
        let Some(enemy) = self.enemy.as_mut()
        else {
            self.message.push("There is no enemy here".to_string());
            return;
        };

        let result = battle_turn(&mut self.player, enemy, action);

        self.message.extend(result.messages);

        match result.outcome {
            Some(BattleOutcome::Victory) => {
                self.message.push("Battle outcome victory".to_string());
                self.enemy = None;

                self.dungeon.advance_room();

                if self.dungeon.is_cleared() {
                    self.reward_dungeon_clear();
                }
            }
            Some(BattleOutcome::Defeat) => {
                self.message.push("Battle outcome defeat".to_string());
                self.should_quit = true;
            }
            Some(BattleOutcome::Quit) => {
                self.message.push("Battle outcome quit".to_string());
                self.should_quit = true;
            }
            Some(BattleOutcome::Fled) => {
                self.message.push("Battle outcome fled".to_string());
                self.enemy = None;
            }
            None => {
                self.message.push("Battle Continues".to_string());
            }
        }
    }

    pub fn reward_dungeon_clear(&mut self) {
        let gold_bonus = self.dungeon_level * 25;
        let xp_bonus = self.dungeon_level * 20;


        self.player.gold += gold_bonus;
        self.player.xp += xp_bonus;
        self.message.extend(check_level_up(&mut self.player));
    }
}
