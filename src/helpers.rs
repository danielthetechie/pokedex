use std::io;

pub fn get_user_input () -> String
{
	let mut user_input = String::new ();
	let mut stdin = io::stdin ();
	stdin.read_line (&mut user_input).expect ("Failed to read line.");

	return user_input.trim ().to_string ();
}