//! A unit's quiet deaths, its biped's flight through the air, its
//! surfaces and its melee, whether it speaks and the equipment it holds
//! (units.c, bipeds.c, unit_dialogue.c and biped_limp_noodle.c).
//!
//! The game's delete, initialize and dispose hooks of units, bipeds and
//! vehicles (`unit_delete`, `bipeds_initialize`, `vehicles_dispose` and
//! the rest) are empty: they have no counterpart here, and the trace
//! shows that each leaves the world as it was.
//!
//! Each is checked against the game's code by
//! tests/unit_life_reference.rs.

/// An object's damage flags (`object_datum.object.damage_flags`): a word
/// of the bits below (objects.h).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DamageFlags(pub u32);

impl DamageFlags {
    /// It is dead (`_object_dead_bit`).
    pub const DEAD: u32 = 1 << 2;
    /// It dies by an act of god, with no sound or effect
    /// (`_object_die_act_of_god_silent_bit`).
    pub const DIE_ACT_OF_GOD_SILENT: u32 = 1 << 6;
    /// It dies by an act of god, counted in nobody's statistics
    /// (`_object_die_act_of_god_no_statistics_bit`).
    pub const DIE_ACT_OF_GOD_NO_STATISTICS: u32 = 1 << 13;

    /// Whether `flag` is set.
    pub fn has(self, flag: u32) -> bool {
        self.0 & flag != 0
    }

    /// Its death will count in nobody's statistics
    /// (`unit_kill_no_statistics`).
    pub fn kill_no_statistics(&mut self) {
        self.0 |= Self::DIE_ACT_OF_GOD_NO_STATISTICS;
    }

    /// Its death will be silent (`unit_kill_silent`).
    pub fn kill_silent(&mut self) {
        self.0 |= Self::DIE_ACT_OF_GOD_SILENT;
    }
}

/// What a biped's flight through the air depends on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BipedAir {
    /// The frames it has been in the air (a signed byte, as the game's).
    pub airborne_ticks: i8,
    /// Its definition flies (`_biped_flying_bit`).
    pub flying: bool,
    /// It is dead.
    pub dead: bool,
}

impl BipedAir {
    /// Whether it is flying through the air: more than three frames in
    /// the air, and either not one that flies or dead
    /// (`biped_flying_through_air`).
    pub fn flying_through_air(self) -> bool {
        self.airborne_ticks > 3 && (!self.flying || self.dead)
    }
}

/// Whether a unit is flying through the air: only a biped can be (none
/// for any other unit) (`unit_flying_through_air`).
pub fn flying_through_air(biped: Option<&BipedAir>) -> bool {
    biped.is_some_and(|biped| biped.flying_through_air())
}

/// The surfaces of the level a biped stands on and finds its paths on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BipedSurfaces {
    /// The surface it stands on.
    pub support: Option<u32>,
    /// The surface it finds its paths on.
    pub pathfinding: Option<u32>,
    /// The surface it last found its paths on.
    pub last_pathfinding: Option<u32>,
}

impl BipedSurfaces {
    /// Forgets every surface, as when the level's structure is unloaded
    /// (`biped_disconnect_from_structure_bsp`).
    pub fn disconnect_from_structure_bsp(&mut self) {
        *self = BipedSurfaces::default();
    }
}

/// Ends a player biped's melee attack: its melee ticks go to 0; no biped,
/// nothing (`biped_stop_melee_attack`).
pub fn stop_melee_attack(melee_ticks: Option<&mut i8>) {
    if let Some(ticks) = melee_ticks {
        *ticks = 0;
    }
}

/// Whether a unit is speaking: what it says now has a priority above none
/// (`unit_is_speaking`).
pub fn is_speaking(current_priority: i16) -> bool {
    current_priority > 0
}

/// The equipment a unit holds (`unit_datum.unit.equipment_object_index`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnitEquipment(pub Option<u32>);

impl UnitEquipment {
    /// The equipment it holds now (`unit_get_current_equipment`).
    pub fn current(self) -> Option<u32> {
        self.0
    }
}

/// The most passes a biped's limp body relaxes its nodes in, whatever the
/// biped (`biped_limp_noodle_get_max_relaxation_iterations`).
pub fn max_relaxation_iterations() -> i32 {
    20
}
