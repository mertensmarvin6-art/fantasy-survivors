//! First iteration of class-specific legendary evolution recipes.
//!
//! This module is deliberately independent of the runtime so we can review
//! progression rules before connecting them to level-up cards and spell casting.
//! All nine original classes remain available in the base game.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvolutionRecipe {
    pub class: &'static str,
    pub name: &'static str,
    pub required_abilities: &'static [(&'static str, u8)],
    pub description: &'static str,
}

pub const SHAMAN_EVOLUTIONS: &[EvolutionRecipe] = &[
    EvolutionRecipe {
        class: "Shaman",
        name: "Elemental Storm",
        required_abilities: &[("lightning_bolt", 3), ("chain_lightning", 3)],
        description: "Lightning strikes can trigger additional chained strikes.",
    },
];

/// Returns true if every prerequisite ability has reached its required rank.
/// An ability that is missing is treated as rank zero.
pub fn is_unlocked(recipe: &EvolutionRecipe, ranks: &[(&str, u8)]) -> bool {
    recipe.required_abilities.iter().all(|(required, minimum)| {
        ranks.iter()
            .filter(|(owned, _)| owned == required)
            .map(|(_, rank)| *rank)
            .max()
            .unwrap_or(0) >= *minimum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolution_requires_both_spells() {
        let recipe = &SHAMAN_EVOLUTIONS[0];
        assert!(!is_unlocked(recipe, &[("lightning_bolt", 3)]));
        assert!(!is_unlocked(recipe, &[("lightning_bolt", 3), ("chain_lightning", 2)]));
        assert!(is_unlocked(recipe, &[("lightning_bolt", 3), ("chain_lightning", 3)]));
    }

    #[test]
    fn missing_or_unrelated_spells_do_not_unlock() {
        assert!(!is_unlocked(&SHAMAN_EVOLUTIONS[0], &[]));
        assert!(!is_unlocked(&SHAMAN_EVOLUTIONS[0], &[("fireball", 9)]));
    }
}
