use std::cmp;

fn sub(value: u32, dec_amount: u32) -> u32 {
    if value > dec_amount {
        value - dec_amount
    } else {
        0
    }
}

#[derive(Clone, Copy)]
enum Effect {
    Shield { timer: u32 },
    Poison { timer: u32 },
    Recharge { timer: u32 },
}

impl Effect {
    fn tick(&self) -> Effect {
        match self {
            Effect::Shield { timer }   => Effect::Shield { timer: sub(*timer, 1) },
            Effect::Poison { timer }   => Effect::Poison { timer: sub(*timer, 1) },
            Effect::Recharge { timer } => Effect::Recharge { timer: sub(*timer, 1) },
        }
    }

    fn to_string(&self) -> String {
        match self {
            Effect::Shield { timer }   => String::from("Shield"),
            Effect::Poison { timer }   => String::from("Poison"),
            Effect::Recharge { timer } => String::from("Recharge"),
        }
    }
}

enum Spell {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u32 {
        match self {
            Spell::MagicMissle => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Spell::MagicMissle => String::from("Magic Missle"),
            Spell::Drain       => String::from("Drain"),
            Spell::Shield      => String::from("Shield"),
            Spell::Poison      => String::from("Poison"),
            Spell::Recharge    => String::from("Recharge"),
        }
    }
}

struct Enemy {
    hp: u32,
    damage: u32,
    poison: Option<Effect>,
}

impl Enemy {
    fn can_receive_spell(&self, spell: &Spell) -> bool {
        match spell {
            Spell::Poison => self.poison.is_none(),
            _ => true
        }
    }

    fn process_effects(&self) -> Enemy {
        match self.poison {
            Some(p) => {
                let next_effect = p.tick();
                match next_effect {
                    Effect::Poison { timer } => {
                        println!("Poison deals 3 damage; its timer is now {}", timer);
                    },
                    _ => {}
                }
                self.receive_damage(3).receive_effect(next_effect)
            },
            _ => Enemy { ..*self }
        }
    }

    fn receive_damage(&self, damage: u32) -> Enemy {
        Enemy {
            hp: sub(self.hp, damage),
            ..*self
        }
    }

    fn receive_effect(&self, effect: Effect) -> Enemy {
        match effect {
            Effect::Poison { timer } => {
                if timer == 0 {
                    Enemy { poison: None, ..*self }
                } else {
                    Enemy { poison: Some(effect), ..*self }
                }
            },
            _ => Enemy { ..*self }
        }
    }

    fn receive_spell(&self, spell: &Spell) -> Enemy {
        match spell {
            Spell::MagicMissle => self.receive_damage(4),
            Spell::Drain => self.receive_damage(2),
            Spell::Poison => self.receive_effect(Effect::Poison { timer: 6 }),
            _ => Enemy { ..*self }
        }
    }
}

struct Player {
    hp: u32,
    mp: u32,
    shield: Option<Effect>,
    recharge: Option<Effect>,
}

impl Player {
    fn cast_spell(&self, spell: &Spell) -> Player {
        let mp = sub(self.mp, spell.cost());
        match spell {
            Spell::Shield   => Player { shield: Some(Effect::Shield { timer: 6 }), mp: mp, ..*self },
            Spell::Drain    => Player { hp: self.hp + 2, mp: mp, ..*self },
            Spell::Recharge => Player { recharge: Some(Effect::Recharge { timer: 5 }), mp: mp, ..*self },
            _ => Player { mp: mp, ..*self }
        }
    }

    fn can_cast_spell(&self, spell: &Spell) -> bool {
        if self.mp < spell.cost() {
            false
        } else {
            match spell {
                Spell::Shield => self.shield.is_none(),
                Spell::Recharge => self.recharge.is_none(),
                _ => true
            }
        }
    }

    fn process_effects(&self) -> Player {
        match self.recharge {
            Some(r) => self.recharge_mana(101).receive_effect(r.tick()),
            _ => Player { ..*self }
        }
    }

    fn receive_damage(&self, damage: u32) -> Player {
        let received_damage = match self.shield {
            Some(_) => cmp::max(1, sub(damage, 7)),
            _ => damage
        };
        Player {
            hp: sub(self.hp, received_damage),
            ..*self
        }
    }

    fn receive_effect(&self, effect: Effect) -> Player {
        match effect {
            Effect::Shield { timer } => {
                if timer == 0 {
                    Player { shield: None, ..*self }
                } else {
                    Player { shield: Some(effect), ..*self }
                }
            },
            Effect::Recharge { timer } => {
                if timer == 0 {
                    Player { recharge: None, ..*self }
                } else {
                    Player { recharge: Some(effect), ..*self }
                }
            },
            _ => Player { ..*self }
        }
    }

    fn recharge_mana(&self, mana: u32) -> Player {
        Player { mp: self.mp + mana, ..*self }
    }
}

enum GameState {
    InProgress,
    PlayerWon,
    PlayerLostZeroHp,
    PlayerLostNotEnoughMana,
    InvalidSpellCast,
}

fn simulate_turn(player: &Player, spell: &Spell, enemy: &Enemy, hard: bool) -> (GameState, Player, Enemy) {
    println!("\n-- Player turn --");
    println!("- Player has {} hit points, {} armor, {} mana", player.hp, if player.shield.is_some() { 7 } else { 0 }, player.mp);
    println!("- Boss has {} hit points", enemy.hp);
    let mut next_player = Player { ..*player };
    let mut next_enemy = Enemy { ..*enemy };
    if hard {
        next_player = next_player.receive_damage(1);
        if next_player.hp == 0 {
            return (GameState::PlayerLostZeroHp, next_player, next_enemy)
        }
    }
    next_player = player.process_effects();
    next_enemy = enemy.process_effects();
    if next_enemy.hp == 0 {
        return (GameState::PlayerWon, next_player, next_enemy)
    }
    if next_player.mp < 53 {
        return (GameState::PlayerLostNotEnoughMana, next_player, next_enemy)
    }
    if !next_player.can_cast_spell(&spell) || !next_enemy.can_receive_spell(&spell) {
        return (GameState::InvalidSpellCast, next_player, next_enemy)
    }
    println!("Player casts {}", spell.to_string());
    next_player = next_player.cast_spell(&spell);
    next_enemy = next_enemy.receive_spell(&spell);
    if next_enemy.hp == 0 {
        return (GameState::PlayerWon, next_player, next_enemy)
    }
    println!("\n-- Boss turn --");
    println!("- Player has {} hit points, {} armor, {} mana", next_player.hp, if next_player.shield.is_some() { 7 } else { 0 }, next_player.mp);
    println!("- Boss has {} hit points", next_enemy.hp);
    next_enemy = next_enemy.process_effects();
    next_player = next_player.process_effects();
    if next_enemy.hp == 0 {
        return (GameState::PlayerWon, next_player, next_enemy)
    }
    println!("Boss attacks for {} damage!", next_enemy.damage);
    next_player = next_player.receive_damage(next_enemy.damage);
    if next_player.hp == 0 {
        (GameState::PlayerLostZeroHp, next_player, next_enemy)
    } else {
        (GameState::InProgress, next_player.receive_damage(next_enemy.damage), next_enemy)
    }
}

fn simulate_battle(player: &Player, spells: &Vec<Spell>, enemy: &Enemy, hard: bool) -> u32 {
    let mut next_player = Player { ..*player };
    let mut next_enemy = Enemy { ..*enemy };
    let mut total_mana_spent = 0;
    for spell in spells.iter() {
        total_mana_spent += spell.cost();
        let next = simulate_turn(&next_player, &spell, &next_enemy, hard);
        next_player = next.1;
        next_enemy = next.2;

        match next.0 {
            GameState::PlayerWon => {
                println!("\nPlayer wins! {} mana spent", total_mana_spent);
                break;
            },
            GameState::PlayerLostZeroHp => {
                println!("\nPlayer loses: 0 hp");
                break;
            },
            GameState::PlayerLostNotEnoughMana => {
                println!("\nPlayer loses: not enough mana ({}) to cast another spell", player.mp);
                break;
            },
            GameState::InvalidSpellCast => {
                println!("\nERROR: {} cannot be cast this turn!", spell.to_string());
                break;
            }
            _ => {}
        }
    }
    total_mana_spent
}

fn main() {
    let mut player = Player { hp: 50, mp: 500, shield: None, recharge: None };
    let mut enemy = Enemy { hp: 55, damage: 8, poison: None };

    // Notes:
    // - Poison is most mana-efficient source of damage, followed by MM
    // - Drain is inefficient mana-wise
    // - Boss wins in 7 swings (unmitigated)
    // - At least 1 Shield required
    // - At least 1 Recharge required

    // NOTE: These solutions can/should be found via BFS. I had enough after
    // working through getting the simulation working properly and just
    // worked it out by hand/trial-and-error.

    let spells_a = vec![
        Spell::Poison {},
        Spell::Recharge {},
        Spell::Shield {},
        Spell::Poison {},
        Spell::MagicMissle {},
        Spell::MagicMissle {},
        Spell::MagicMissle {},
        Spell::MagicMissle {},
        Spell::MagicMissle {},
    ];

    let spells_b = vec![
        Spell::Poison {},
        Spell::Recharge {},
        Spell::Shield {},
        Spell::Poison {},
        Spell::Recharge {},
        Spell::Drain {},
        Spell::Poison {},
        Spell::Drain {},
        Spell::MagicMissle {},
    ];

    println!("A: {}", simulate_battle(&player, &spells_a, &enemy, false));
    println!("B: {}", simulate_battle(&player, &spells_b, &enemy, true));
}
