#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Clone)]
enum PokeType 
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
struct Pokemon 
{
	name: String,
	ptype: PokeType,
	attacks: [String; 2]
}

struct Pokedex<'a> {
	pokemons: Vec::<&'a Pokemon>
}

impl<'a> Pokedex<'a> 
{	
	fn get_pokemon (&self, pokename: &str) -> Option<&&Pokemon>
	{
		let pokemon_option = self.pokemons.iter ().find (|p| p.name == pokename);
		return pokemon_option;
	}

	fn remove_pokemon (&mut self, pokemon: &Pokemon) -> ()
	{
		self.pokemons.retain (|&p| p != pokemon);
	}

	fn print_pokemon (&self, pokename: &str)
	{
		let pokemon_option = self.get_pokemon (&pokename);

		match pokemon_option
		{
			Some (pokemon) => 
			{
				println! ("Name: {}", pokemon.name);
				println! ("Type: {}", pokemon.ptype);

				print! ("Attacks: ");
				for a in &pokemon.attacks
				{
					print! ("{}, ", a);
				}
				println! ("");
			},
			None => println!("Pokemon not found")
		};
	}

	fn print_all_pokemon_names (&self)
	{
		for poke in &self.pokemons
		{
			println! ("{}", poke.name);
		}
	}

	fn register_new_pokemon (&mut self, pokemon: &'a Pokemon, print_info: bool)
	{
		let pokemon_option = self.get_pokemon (&pokemon.name);
		match pokemon_option
		{
			Some (pokeresult) => println! ("{} already exists!", &pokemon.name),
			None => 
			{
				self.pokemons.push (pokemon);
				println! ("{} registered!", &pokemon.name);
				if print_info
				{
					self.print_pokemon (&pokemon.name);
				}
			}
		};
	}
}

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

	//pokedex.print_pokemon (String::from ("Charizard"));

	pokedex.register_new_pokemon (&dugtrio, false);
	pokedex.remove_pokemon (&charizard);
	pokedex.print_all_pokemon_names ();
}