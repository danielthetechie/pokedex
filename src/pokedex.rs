use crate::Pokemon;
use crate::get_user_input;

pub struct Pokedex<'a> {
	pub pokemons: Vec::<&'a Pokemon>
}

impl<'a> Pokedex<'a> 
{	
	fn get_pokemon (&self, pokename: &str) -> Option<&&Pokemon>
	{
		let pokemon_option = self.pokemons.iter ().find (|p| p.name == pokename);
		return pokemon_option;
	}

	pub fn remove_pokemon (&mut self, pokemon: &Pokemon) -> ()
	{
		self.pokemons.retain (|&p| p != pokemon);
	}

	pub fn print_pokemon (&self, pokename: &str)
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

	pub fn print_all_pokemon_names (&self)
	{
		for poke in &self.pokemons
		{
			println! ("{}", poke.name);
		}
	}

	pub fn register_new_pokemon (&mut self, pokemon: &'a Pokemon, print_info: bool)
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

	pub fn choose_action (&self)
	{
		let print_options = "Choose an option:\n\n\t1: Print all pokemons\n\t2: Get pokémon info";
		println! ("{}", print_options);

		let mut option = get_user_input ();
		let option_number: u8 = option.parse::<u8>().expect ("Not a number!");

		match option_number 
		{
			1 => self.print_all_pokemon_names (),
			2 => 
			{
				println! ("Write the pokémon name you want to check: ");
				option = get_user_input ();
				self.print_pokemon (&option);
			}
			_ => 
			{
				println! ("Choose a valid option, dumbass.");
				self.choose_action ();
			}
		}
	}
}