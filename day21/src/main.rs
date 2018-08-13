struct Entity {
    hp: u32,
    base_damage: u32,
    base_armor: u32,
}

struct Weapon {
    name: String,
    cost: u32,
    damage: u32,
}

struct Armor {
    name: String,
    cost: u32,
    value: u32,
}

struct Ring {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

struct Inventory<'a> {
    weapon: &'a Weapon,
    armor: Option<&'a Armor>,
    ring1: Option<&'a Ring>,
    ring2: Option<&'a Ring>,
}

impl<'a> Inventory<'a> {
    fn total_cost(&self) -> u32 {
        self.weapon.cost +
            self.armor.map(|a| a.cost).unwrap_or_default() +
            self.ring1.map(|r| r.cost).unwrap_or_default() +
            self.ring2.map(|r| r.cost).unwrap_or_default()
    }

    fn total_damage(&self) -> u32 {
        self.weapon.damage +
            self.ring1.map(|r| r.damage).unwrap_or_default() +
            self.ring2.map(|r| r.damage).unwrap_or_default()
    }

    fn total_armor(&self) -> u32 {
        self.armor.map(|a| a.value).unwrap_or_default() +
            self.ring1.map(|r| r.armor).unwrap_or_default() +
            self.ring2.map(|r| r.armor).unwrap_or_default()
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.weapon.name);
        result.push_str(" / ");
        match self.armor {
            Some(ar) => {
                result.push_str(&ar.name);
            },
            None => {
                result.push_str("-- ")
            }
        };
        result.push_str(" / ");
        match self.ring1 {
            Some(r) => {
                result.push_str(&r.name);
            },
            None => {
                result.push_str("-- ")
            }
        };
        result.push_str(" / ");
        match self.ring2 {
            Some(r) => {
                result.push_str(&r.name);
            },
            None => {
                result.push_str("--")
            }
        };
        result
    }
}

fn simulate_turn(player: &Entity, inv: &Inventory, enemy: &Entity) -> (Entity, Entity) {
    let damage_to_enemy = if inv.total_damage() > enemy.base_armor {
        inv.total_damage() - enemy.base_armor
    } else {
        1
    };
    let next_enemy = Entity {
        hp: if enemy.hp > damage_to_enemy { enemy.hp - damage_to_enemy } else { 0 },
        ..*enemy
    };
    if next_enemy.hp == 0 {
        (Entity { ..*player }, next_enemy)
    } else {
        let damage_to_player = if enemy.base_damage > inv.total_armor() {
            enemy.base_damage - inv.total_armor()
        } else {
            1
        };
        let next_player = Entity {
            hp: if player.hp > damage_to_player { player.hp - damage_to_player } else { 0 },
            ..*player
        };
        (next_player, next_enemy)
    }
}

fn simulate_battle(player: &Entity, inventory: &Inventory, enemy: &Entity) -> bool {
    let mut next_player = Entity { ..*player };
    let mut next_enemy = Entity { ..*enemy };
    while next_player.hp > 0 && next_enemy.hp > 0 {
        let result = simulate_turn(&next_player, &inventory, &next_enemy);
        next_player = result.0;
        next_enemy = result.1;
    }
    next_player.hp > 0
}

fn main() {
    let player = Entity { hp: 100, base_damage: 0, base_armor: 0 };
    let enemy = Entity { hp: 103, base_damage: 9, base_armor: 2 };

    let weapons = [
        Weapon { name: String::from("Dagger"),     cost:  8, damage: 4 },
        Weapon { name: String::from("Shortsword"), cost: 10, damage: 5 },
        Weapon { name: String::from("Warhammer"),  cost: 25, damage: 6 },
        Weapon { name: String::from("Longsword"),  cost: 40, damage: 7 },
        Weapon { name: String::from("Greataxe"),   cost: 74, damage: 8 },
    ];

    let armors = [
        Armor { name: String::from("Leather"),    cost:  13, value: 1 },
        Armor { name: String::from("Chainmail"),  cost:  31, value: 2 },
        Armor { name: String::from("Splintmail"), cost:  53, value: 3 },
        Armor { name: String::from("Bandedmail"), cost:  75, value: 4 },
        Armor { name: String::from("Platemail"),  cost: 102, value: 5 },
    ];

    let rings = [
        Ring { name: String::from("Damage +1"),  cost:  25, damage: 1, armor: 0 },
        Ring { name: String::from("Damage +2"),  cost:  50, damage: 2, armor: 0 },
        Ring { name: String::from("Damage +3"),  cost: 100, damage: 3, armor: 0 },
        Ring { name: String::from("Defense +1"), cost:  20, damage: 0, armor: 1 },
        Ring { name: String::from("Defense +2"), cost:  40, damage: 0, armor: 2 },
        Ring { name: String::from("Defense +3"), cost:  80, damage: 0, armor: 3 },
    ];

    let mut inventories = Vec::new();

    for weapon in weapons.iter() {
        inventories.push(Inventory { weapon: &weapon, armor: None, ring1: None, ring2: None });

        for armor in armors.iter() {
            inventories.push(Inventory { weapon: &weapon, armor: Some(&armor), ring1: None, ring2: None });

            for (i, ring1) in rings.iter().enumerate() {
                inventories.push(Inventory { weapon: &weapon, armor: Some(&armor), ring1: Some(&ring1), ring2: None });

                for ring2 in rings.iter().skip(i + 1) {
                    inventories.push(Inventory { weapon: &weapon, armor: Some(&armor), ring1: Some(&ring1), ring2: Some(&ring2) });
                }
            }
        }

        for (i, ring1) in rings.iter().enumerate() {
            inventories.push(Inventory { weapon: &weapon, armor: None, ring1: Some(&ring1), ring2: None });

            for ring2 in rings.iter().skip(i + 1) {
                inventories.push(Inventory { weapon: &weapon, armor: None, ring1: Some(&ring1), ring2: Some(&ring2) });
            }
        }
    }

    let mut cheapest_winning_inv: Option<&Inventory> = None;
    let mut most_expensive_losing_inv: Option<&Inventory> = None;

    for inv in inventories.iter() {
        let cost = inv.total_cost();
        if simulate_battle(&player, &inv, &enemy) {
            match cheapest_winning_inv {
                Some(i) => {
                    if cost < i.total_cost() {
                        cheapest_winning_inv = Some(inv);
                    }
                },
                None => {
                    cheapest_winning_inv = Some(inv);
                }
            }
        } else {
            match most_expensive_losing_inv {
                Some(i) => {
                    if cost > i.total_cost() {
                        most_expensive_losing_inv = Some(inv);
                    }
                },
                None => {
                    most_expensive_losing_inv = Some(inv);
                }
            }
        }
    }

    let inv_a = cheapest_winning_inv.unwrap();
    let inv_b = most_expensive_losing_inv.unwrap();
    println!("A: {} ({})", inv_a.total_cost(), inv_a.to_string());
    println!("B: {} ({})", inv_b.total_cost(), inv_b.to_string());
}
