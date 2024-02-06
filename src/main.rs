#![allow(dead_code)]
#![allow(unused_variables)]

mod helpers;
mod pokemon;
mod pokedex;

use helpers::get_user_input;
use pokemon::{Pokemon, PokeType};
use pokedex::Pokedex;

fn main () -> ()
{
	let charizard = Pokemon 
	{
		name: String::from ("Charizard"),
		ptype: PokeType::Fire,
		attacks: [String::from ("Flamethrower"), String::from ("Wing attack")]
	};

	let blastoise = Pokemon 
	{
		name: String::from ("Blastoise"),
		ptype: PokeType::Water,
		attacks: [String::from ("Surf"), String::from ("Hydro-pump")]
	};

	let venusaur = Pokemon 
	{
		name: String::from ("Venusaur"),
		ptype: PokeType::Plant,
		attacks: [String::from ("Solar beam"), String::from ("Vine whip")]
	};

	let dugtrio = Pokemon 
	{
		name: String::from ("Dugtrio"),
		ptype: PokeType::Earth,
		attacks: [
			String::from ("Earthquake"),
			String::from ("Slash")
		]
	};

	let mut pokedex = Pokedex 
	{
		pokemons: vec![&charizard, &blastoise, &venusaur]
	};

	pokedex.choose_action ();
}