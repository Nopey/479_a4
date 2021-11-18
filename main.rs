//! hfkb
//!
//! This file contains the entrypoint
//! 
//! Magnus Larsen 2021

mod hfkb;

use hfkb::HornFormKb;

/// Handles commandline interface and program lifecycle
fn main() {
	println!("## Rust HFKB (main.rs)");
	let mut my_kb = HornFormKb::new();
	let my_query = "F";

	my_kb.tell("A^B^C", "D");
	my_kb.tell("AB^E",  "F");
	my_kb.tell("P1^P2", "B");
	my_kb.tell("", "A");
	my_kb.tell("", "P1");
	my_kb.tell("", "E");

	if my_kb.ask(my_query) {
		println!("My knowledge entails {}", my_query);
	} else {
		println!("My knowledge base does not entail {}", my_query);
	}
}
