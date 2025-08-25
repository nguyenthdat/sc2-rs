use sc2::prelude::*;

// Example of how to use events

#[bot]
#[derive(Default)]
struct EmptyBot;
impl Player for EmptyBot {
	fn get_player_settings(&'_ self) -> PlayerSettings<'_> {
		PlayerSettings::new(Race::Random)
	}

	// Use it like here
	fn on_event(&mut self, event: Event) -> SC2Result<()> {
		match event {
			Event::UnitDestroyed(_tag, alliance) => {
				match alliance {
					Some(Alliance::Own) => { /* your code here */ }
					Some(Alliance::Neutral) => { /* your code here */ }
					// Enemy
					_ => { /* your code here */ }
				}
			}
			Event::UnitCreated(tag) => {
				if let Some(_u) = self.units.my.units.get(tag) { /* your code here */ }
			}
			Event::ConstructionStarted(tag) => {
				if let Some(_u) = self.units.my.structures.get(tag) { /* your code here */ }
			}
			Event::ConstructionComplete(tag) => {
				if let Some(_u) = self.units.my.structures.get(tag) { /* your code here */ }
			}
			Event::RandomRaceDetected(_race) => { /* your code here */ }
		}
		Ok(())
	}
}

fn main() -> SC2Result<()> {
	let mut bot = EmptyBot::default();

	run_vs_computer(
		&mut bot,
		Computer::new(Race::Random, Difficulty::VeryEasy, None),
		"EverDreamLE",
		Default::default(),
	)
}
