use crate::{
	effects::{
		context::EffectContext,
		traits::{Effect, GameEvent},
	},
	types::{
		card::{CardAsset, Suit},
		deck::Deck,
		game::Game,
		tower::Tower,
	},
};

pub struct HeartHeal;

impl Effect for HeartHeal {
	fn name() -> &'static str {
		"Heart Heal"
	}

	fn description() -> &'static str {
		"Heals player for the sum of the ranks of all hearts in the attack"
	}

	fn add(_: Game, _: Deck, _: Tower, _: u8, _: EffectContext) {
		// no effect on add
	}

	fn remove(_: Game, _: Deck, _: Tower, _: u8, _: EffectContext) {
		// no effect on remove
	}

	fn apply(
		game_event: GameEvent,
		game: &mut Game,
		_deck: &mut Deck,
		_tower: &mut Tower,
		_level: u8,
		context: EffectContext,
	) {
		if game_event != GameEvent::OnAttack {
			return;
		}

		if let EffectContext::Attack(att) = context {
            let heal_amount = att
                .cards
                .into_iter()
                .map(|index| CardAsset::try_from(index as u32).unwrap())
                .filter(|c| c.suit == Suit::Hearts)
                .fold(0, |v, card| v + card.rank.as_value());


            let new_damage = game.player.player_damage.saturating_sub(heal_amount);
            game.player.player_damage = new_damage;
        }
	}
}
