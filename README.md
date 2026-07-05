# CLI RPG

A small terminal-based RPG written in Rust. You create a player, enter the dungeon, fight randomly generated enemies,
collect gold and XP, and level up as long as you survive.

## Features

- Simple command-line gameplay
- Random enemy encounters
- Enemy levels that scale around the player level
- Enemy rarity tiers: Common, Rare, Elite, and Boss
- Enemy modifiers such as Angry, Wild, Cursed, Ancient, Hungry, Dark, and Broken
- Gold and XP rewards after winning battles
- Level-up system with increased health and damage
- Healing, attacking, running away, and quitting from battle

## Requirements

You need Rust and Cargo installed.

Check your installation with:

```bash
rustc --version
cargo --version
```

If Rust is not installed, install it from:

https://www.rust-lang.org/tools/install

## How To Play

Clone the project and enter the folder:

```bash
git clone https://github.com/HDemir23/Rust-Cli-Rpg.git
cd Cli_Rpg
```

Run the game:

```bash
cargo run
```

You will first be asked for your name:

```text
Enter Ur name
```

After that, the game starts generating dungeon encounters.

## Battle Commands

During a battle, the game shows this prompt:

```text
command: attack => a / heal => e / run => r / quit q
```

Use one of these commands:

| Command | Action                           |
|---------|----------------------------------|
| `a`     | Attack the enemy                 |
| `e`     | Heal yourself                    |
| `r`     | Run away from the current battle |
| `q`     | Quit the current battle          |

Invalid commands do not spend the enemy turn. The game will print an unknown command message and ask again.

## Continuing The Dungeon

After each encounter, if your player is still alive, the game asks:

```text
continue playing y / n
```

Use:

| Command | Action                     |
|---------|----------------------------|
| `y`     | Continue to the next enemy |
| `n`     | Exit the dungeon           |

Any other answer is treated as invalid and ends the dungeon run.

## Player Stats

The player starts with:

| Stat                  | Starting Value             |
|-----------------------|----------------------------|
| Health                | `100`                      |
| Max health            | `100`                      |
| Heal amount           | `25` plus 5% of max health |
| Damage                | `20`                       |
| Gold                  | `0`                        |
| Level                 | `1`                        |
| XP                    | `0`                        |
| XP needed for level 2 | `35`                       |

When you level up:

- Your level increases by `1`
- Max health increases by `20`
- Health is fully restored
- Damage increases by `5`
- The XP needed for the next level increases

## Enemies

Enemies are randomly generated from these base enemy types:

- Goblin
- Skeleton
- Orc
- Wolf

Each enemy gets:

- A level near your current level
- A rarity tier
- A name modifier
- Scaled health and damage
- Gold and XP rewards

Example encounter:

```text
A wild Dark Orc appeared!
Level: 2
Rarity: Rare
Health: 110
Damage: 28
```

## Enemy Rarity

Rarity affects enemy strength and rewards.

| Rarity | What It Means                              |
|--------|--------------------------------------------|
| Common | Normal enemy                               |
| Rare   | Stronger enemy with better rewards         |
| Elite  | Much stronger enemy with high rewards      |
| Boss   | Very dangerous enemy with the best rewards |

## Winning And Losing

You win a battle when the enemy health reaches `0` or below. You receive gold and XP, and the game checks whether you
level up.

You lose the run when your health reaches `0` or below. The game prints your final gold and XP.

## Build A Release Version

For a faster standalone build:

```bash
cargo build --release
```

Run the release binary:

```bash
./target/release/Cli_Rpg
```

On Windows, the binary is:

```bash
target\release\Cli_Rpg.exe
```

## Project Structure

```text
src/
  main.rs                 Game startup and main dungeon loop
  dungeon.rs              Dungeon module placeholder
  battle.rs               Battle commands, turns, damage, healing, and rewards
  enemy_gen.rs            Random enemy generation, rarity, and modifiers
  player_progression.rs   Healing amount and level-up logic
  read_input.rs           Terminal input helper
  structs.rs              Player, enemy, rarity, and turn result types
Cargo.toml                Rust package metadata and dependencies
Cargo.lock                Locked dependency versions
```

## Troubleshooting

If `cargo run` does not work, check that Rust is installed:

```bash
rustc --version
cargo --version
```

If Cargo cannot find the project, make sure you are inside the project folder that contains `Cargo.toml`.

If the game closes after entering a command, check whether you typed `n`, `q`, or an invalid answer at the continue
prompt.

## Current Limitations

- There is no save system yet.
- There is no inventory or equipment system yet.
- The game is text-only.
- Running away ends the current battle but does not give rewards.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
