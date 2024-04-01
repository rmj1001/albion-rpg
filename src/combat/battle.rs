use super::inventory::battle_menu;
use crate::{
    combat::enemy::{Data, Rewards},
    data::{inventory::equipment::Equipment, player::Player, xp::XP},
    panic_menu,
    prelude::{confirm, page_header, pause, random_num, select, sleep, unreachable, Instructions},
};

pub struct Battle<'a> {
    pub header: &'static str,
    pub prompt: &'static str,
    pub player: &'a mut Player,
    pub enemy: Data,
    pub loops: usize,
    pub floor: usize,
    pub is_first_battle: bool,
    pub is_looped: bool,
    pub pause_seconds: u64,
    pub end_function: Option<fn(&mut Player)>,
}

/**
--------------------------------------------------------------------------------
Entrypoint and Main Battle Menu
--------------------------------------------------------------------------------
*/
impl<'a> Battle<'a> {
    pub fn new(
        title: &'static str,
        prompt: &'static str,
        player: &'a mut Player,
        loops: usize,
        exit_function: Option<fn(&mut Player)>,
    ) -> Self {
        Self {
            header: title,
            prompt,
            enemy: Data::new(player.xp.combat, player.health.hp),
            player,
            loops,
            floor: 0,
            is_first_battle: true,
            is_looped: loops > 0,
            pause_seconds: 1,
            end_function: exit_function,
        }
    }

    pub fn start(&mut self) {
        // Prelude
        page_header(self.header, &Instructions::None);
        Equipment::check_equipment_ownership(self.player);

        if self.player.equipment.armor.is_none() || self.player.equipment.weapon.is_none() {
            let confirm = confirm("Are you sure you want to fight without equipment? You'll die.");

            if !confirm {
                println!("Returning home.");
                pause();

                crate::menus::game_menu::main(self.player);
            }
        }

        if self.loops > 0 {
            self.floor += 1;
            self.loops -= 1;
        }

        println!("{}", self.prompt);
        sleep(self.pause_seconds);

        if self.is_first_battle {
            self.is_first_battle = false; // generate new enemy for subsequent battles
        } else {
            self.enemy = Data::new(self.player.xp.combat, self.player.health.hp);
        }

        println!();
        println!("You are now fighting a {}.", self.enemy.name);
        sleep(self.pause_seconds);
        self.main_menu();
    }

    fn main_menu(&mut self) {
        page_header(
            format!("{} - {}", self.header, self.enemy.name),
            &Instructions::Keyboard,
        );

        if self.is_looped {
            println!("Floor: {}", self.floor);
            println!("Floors Left: {}", self.loops);
            println!();
        }

        println!("{}", self.enemy);

        println!("Player HP: {}", self.player.health.hp);
        println!("Player Hunger: {}", self.player.health.hunger);
        println!();

        let attack_string = &format!("1. Attack the {}", self.enemy.name);

        let action = select(&[attack_string.as_str(), "2. Inventory", "3. Retreat"], None);

        match action {
            0 => self.attack(),
            1 => {
                battle_menu(self.player);
                self.main_menu();
            }
            2 => self.retreat(),
            _ => unreachable(),
        }
    }
}

/**
--------------------------------------------------------------------------------
Attacking Sequences
--------------------------------------------------------------------------------
*/
impl<'a> Battle<'a> {
    fn attack(&mut self) {
        page_header(self.header, &Instructions::None);

        self.player_attack();

        println!();

        self.enemy_attack();

        println!();

        self.player.health.heal();

        println!();

        pause();

        self.main_menu();
    }

    fn player_attack(&mut self) {
        let enemy_type = &self.enemy.name;

        println!("You attack the {enemy_type}...");
        sleep(self.pause_seconds);

        if !Self::hit() || self.player.equipment.weapon.is_none() {
            println!("You missed the {enemy_type}.");
            sleep(self.pause_seconds);
            return;
        }

        if let Some(equipped_weapon) = &self.player.equipment.weapon {
            let weapon = self.player.weapons.get(equipped_weapon);
            let damage = weapon.damage;

            println!("You hit the {enemy_type} for {damage} damage!");

            weapon.decrease_durability();

            if !weapon.owns {
                Equipment::unequip_weapon(self.player, false);
            }

            if self.enemy.hp < damage {
                self.victory();
            } else {
                self.enemy.hp -= damage;
            }
        }

        sleep(self.pause_seconds);
    }

    fn enemy_attack(&mut self) {
        let enemy_type = &self.enemy.name;
        let mut damage: usize = self.enemy.damage;

        if let Some(equipped_armor) = &self.player.equipment.armor {
            let armor = self.player.armor.get(equipped_armor);

            if damage > armor.defense {
                damage -= armor.defense;
            } else {
                damage = 0;
            }

            armor.decrease_durability();

            if !armor.owns {
                Equipment::unequip_armor(self.player, false);
            }
        }

        println!("The {enemy_type} attacks you...");
        sleep(self.pause_seconds);

        if Self::hit() && damage > 0 {
            println!("The {enemy_type} hit you for {damage} damage!!");

            if self.player.health.hp < damage {
                self.defeat();
            } else {
                self.player.health.hp -= damage;
            }
        } else if damage == 0 {
            println!("The {enemy_type} hit but the damage was negated by your armor!");
        } else {
            println!("The {enemy_type} missed you.");
        }

        sleep(self.pause_seconds);
    }

    /// Determines if the target of an attack was hit or not.
    fn hit() -> bool {
        random_num(0, 1) == 0
    }
}

/**
--------------------------------------------------------------------------------
Victory, Defeat, Retreat
--------------------------------------------------------------------------------
*/
impl<'a> Battle<'a> {
    fn retreat(&mut self) {
        page_header("Battle - Retreat", &Instructions::None);

        println!("You have retreated from the battle.");
        pause();

        crate::menus::game_menu::main(self.player);
    }

    fn victory(&mut self) {
        page_header(format!("{} - Victory", self.header), &Instructions::None);

        println!("You successfully defeated the {}!", self.enemy.name);
        self.player.health.restore();
        self.player.achievements.monsters_killed += 1;
        println!();

        let rewards = Rewards::new(XP::get_level(self.player.xp.total()));

        println!("Items Looted:");

        for reward in &rewards {
            println!("- {reward}");
        }

        Rewards::reward_to_player(self.player, rewards);
        println!();

        pause();
        self.player.save();

        if !self.is_looped {
            crate::menus::game_menu::main(self.player);
        }

        if self.loops > 0 {
            self.start();
        }

        if let Some(end_func) = self.end_function {
            end_func(self.player);
        }
    }

    fn defeat(&mut self) {
        page_header(format!("{} - Defeat", self.header), &Instructions::None);

        println!("You have been defeated in battle.\n");
        sleep(self.pause_seconds);

        println!("You have been rushed to the local physician.\n");
        sleep(self.pause_seconds);

        if self.player.settings.hardmode {
            self.hardmode();
        } else {
            self.revived();
        }
    }

    fn revived(&mut self) {
        println!("You were successfully revived with 100 hp.\n");
        self.player.health.reset();

        self.player.save();
        pause();

        crate::menus::game_menu::main(self.player);
    }

    /// Result of battle if player defeated and hardmode is enabled.
    fn hardmode(&mut self) {
        let user_survives = random_num(0, 1);

        match user_survives {
            0 => {
                println!(
                    "The {} stole all your gold and inventory, and you lost all your progress.\n",
                    self.enemy.name
                );
                self.player.die();
                sleep(self.pause_seconds);

                self.revived();
            }
            1 => {
                println!("You didn't survive. This profile will be deleted.\n");
                pause();

                match self.player.delete() {
                    Ok(()) => crate::menus::accounts::main(),
                    Err(error) => panic_menu!(error),
                }
            }
            _ => unreachable(),
        }
    }
}
