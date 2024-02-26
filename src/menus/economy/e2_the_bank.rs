use crate::data::inventory::bank::Bank;
use crate::data::player::Player;

pub fn main(player: &mut Player) {
    Bank::menu(player, false);
    crate::menus::game_menu::main(player);
}
