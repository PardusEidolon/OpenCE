//! Whether it is safe to save the game or to speak (`game_all_quiet`,
//! `game_safe_to_save`, `game_safe_to_speak`): the dangers the game weighs
//! before a checkpoint and before a line of dialogue, combined as it
//! combines them.

/// The dangers around the players this tick: the game's eight
/// predicates, in the order `game_safe_to_save` weighs them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dangers {
    /// An enemy can see a player.
    pub enemies_see_player: bool,
    /// A live projectile is near a player.
    pub projectiles_near_player: bool,
    /// A grenade or other dangerous item is near a player.
    pub items_near_player: bool,
    /// A damaging effect is near a player.
    pub effects_near_player: bool,
    /// A unit is throwing a grenade or falling as it dies.
    pub unit_is_dangerous: bool,
    /// A player is in the air or out of the map.
    pub player_in_the_air: bool,
    /// A player is dead.
    pub player_dead: bool,
    /// A vehicle is moving near a player.
    pub vehicle_moving_near_player: bool,
}

impl Dangers {
    /// Nothing is fighting near the players (`game_all_quiet`).
    pub fn all_quiet(&self) -> bool {
        !(self.projectiles_near_player
            || self.items_near_player
            || self.effects_near_player
            || self.unit_is_dangerous
            || self.enemies_see_player)
    }

    /// A checkpoint may be saved: all is quiet, and every player is alive,
    /// on the ground and clear of moving vehicles (`game_safe_to_save`).
    pub fn safe_to_save(&self) -> bool {
        self.all_quiet()
            && !self.player_in_the_air
            && !self.player_dead
            && !self.vehicle_moving_near_player
    }

    /// Dialogue may play: nothing is flying at the players and every player
    /// is alive (`game_safe_to_speak`).
    pub fn safe_to_speak(&self) -> bool {
        !(self.projectiles_near_player || self.player_dead)
    }
}

/// Whether a game is played by more than one player (`game_is_cooperative`).
pub fn is_cooperative(players: usize) -> bool {
    players > 1
}
