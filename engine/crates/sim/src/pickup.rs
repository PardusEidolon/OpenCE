//! What a player on foot does with a weapon lying within its reach, as the
//! game chooses (`player_examine_nearby_item`): takes it without asking,
//! offers to swap it for the one in its hands or to add it to them, or
//! leaves it.
//!
//! The game takes a weapon without asking (`unit_should_autopick_weapon`)
//! when it carries no weapon that counts, when it carries none of its
//! kind and fewer than two that count, or when the weapon does not count
//! toward the most it carries. Otherwise the action button may swap it for
//! the one in its hands (`unit_approve_weapon_swap`) unless it carries one
//! of the kind already, the one in its hands excepted when that one is
//! older (a plasma weapon's spent battery) than the one lying there.

use opence_content::Weapon;

/// A weapon, carried or lying, as the choice sees it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seen {
    /// Its kind (the armory's place for it).
    pub kind: usize,
    /// Its definition's flags ([`Weapon::MUST_BE_READIED`] and the rest).
    pub flags: u32,
    /// Its age, 0 to 1.
    pub age: f32,
}

impl Seen {
    /// Whether it counts toward the most weapons carried.
    fn counts(&self) -> bool {
        self.flags & Weapon::DOESNT_COUNT_TOWARD_MAXIMUM == 0
    }
}

/// What the player does with a weapon lying within reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    /// Leaves it.
    Leave,
    /// Takes it without asking.
    Take,
    /// Offers to add it to the one in its hands (the action button).
    Add,
    /// Offers to swap it for the one in its hands (the action button).
    Swap,
}

/// What a player carrying `carried` (its inventory's places, `None` for
/// an empty one), with `current` in its hands, does with `lying`: with
/// `trigger` held a weapon that must be readied is left, and `forced`
/// takes it without asking (a CTF flag).
pub fn choose(
    carried: &[Option<Seen>],
    current: Option<usize>,
    lying: &Seen,
    trigger: bool,
    forced: bool,
) -> Choice {
    let counting = carried
        .iter()
        .flatten()
        .filter(|seen| seen.counts())
        .count();
    let in_hands = current.and_then(|place| carried.get(place).copied().flatten());
    let in_hands_does_not_count =
        counting >= 2 && lying.counts() && in_hands.is_some_and(|seen| !seen.counts());
    if trigger && lying.flags & Weapon::MUST_BE_READIED != 0 {
        return Choice::Leave;
    }
    // (`unit_approve_weapon_pickup`: it carries none of the kind)
    let none_of_the_kind = carried.iter().flatten().all(|seen| seen.kind != lying.kind);
    if (none_of_the_kind && !lying.counts())
        || counting == 0
        || (none_of_the_kind && counting < 2)
        || forced
    {
        return Choice::Take;
    }
    if in_hands_does_not_count || !swap_approved(carried, current, in_hands, lying) {
        return Choice::Leave;
    }
    match in_hands {
        Some(seen) if counting == 1 && seen.kind != lying.kind => Choice::Add,
        _ => Choice::Swap,
    }
}

/// Whether the action button may swap `lying` for the one in its hands
/// (`unit_approve_weapon_swap`): it has one in its hands, and of the kind
/// it carries only the one in its hands, aged, and older than `lying`.
fn swap_approved(
    carried: &[Option<Seen>],
    current: Option<usize>,
    in_hands: Option<Seen>,
    lying: &Seen,
) -> bool {
    if in_hands.is_none() {
        return false;
    }
    carried.iter().enumerate().all(|(place, seen)| match seen {
        // (a NaN age refuses, as it does in the game)
        Some(seen) if seen.kind == lying.kind => {
            Some(place) == current && seen.age > 0.0 && lying.age < seen.age
        }
        _ => true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(kind: usize) -> Seen {
        Seen {
            kind,
            flags: 0,
            age: 0.0,
        }
    }

    #[test]
    fn a_free_hand_takes_a_new_kind_and_full_hands_offer_a_swap() {
        let one = [Some(seen(0)), None];
        assert_eq!(choose(&one, Some(0), &seen(1), false, false), Choice::Take);
        let two = [Some(seen(0)), Some(seen(1))];
        assert_eq!(choose(&two, Some(0), &seen(2), false, false), Choice::Swap);
        assert_eq!(choose(&two, Some(0), &seen(1), false, false), Choice::Leave);
    }

    #[test]
    fn a_fresher_battery_of_the_kind_in_hand_may_be_swapped() {
        let mut spent = seen(1);
        spent.age = 0.75;
        let mut fresh = seen(1);
        fresh.age = 0.25;
        let two = [Some(seen(0)), Some(spent)];
        assert_eq!(choose(&two, Some(1), &fresh, false, false), Choice::Swap);
        assert_eq!(choose(&two, Some(0), &fresh, false, false), Choice::Leave);
    }

    #[test]
    fn a_weapon_that_must_be_readied_is_left_while_firing() {
        let mut readied = seen(2);
        readied.flags = Weapon::MUST_BE_READIED;
        let none: [Option<Seen>; 2] = [None, None];
        assert_eq!(choose(&none, None, &readied, true, false), Choice::Leave);
        assert_eq!(choose(&none, None, &readied, false, false), Choice::Take);
    }
}
