use std::fmt;

#[derive(Clone)]
pub enum PokeType 
{
	Fire,
	Water,
	Plant,
	Earth
}

impl fmt::Display for PokeType
{
	fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
		match self 
		{
			PokeType::Fire => write! (f, "Fire"),
			PokeType::Water => write! (f, "Water"),
			PokeType::Plant => write! (f, "Plant"),
			PokeType::Earth => write! (f, "Earth")
		}
	}
}

impl PartialEq for Pokemon {
	fn eq (&self, other: &Self) -> bool {
			self.name == other.name
	}
}

#[derive(Clone)]
pub struct Pokemon 
{
	pub name: String,
	pub ptype: PokeType,
	pub attacks: [String; 2]
}