#[link(name = "Tourney",
       vers = "0.1",
       description = "Round-robin tournament scheduler",
       author = "Arcterus",
       license = "MPL v2.0")];

use std::os;
use std::libc::*;

pub enum Gender {
	Male = 0,
	Female,
	Other  /* :) */
}

pub struct Player {
	name: ~str,
	gender: Gender,
	skill: int,
	id: int,
	wins: ~[@mut Team],
	priv partners: ~[@mut Player],
}

pub struct Team {
	p1: @mut Player,
	p2: @mut Player
}

impl Team {
	pub fn new(p1: @mut Player, p2: @mut Player) -> @Team {
		@Team {
			p1: p1,
			p2: p2
		}
	}
	
	pub fn dissolve(&self) {
		self.p1.partners.pop().partners.pop();
	}
}

impl Eq for Player {
	fn eq(&self, other: &Player) -> bool {
		self.id == other.id
	}

	fn ne(&self, other: &Player) -> bool {
		!self.eq(other)
	}
}

impl Player {
	pub fn new(name: ~str, gender: Gender,
	           skill: int, id: int) -> @Player {
		@Player {
			name: name,
			gender: gender,
			skill: skill,
			id: id,
			wins: ~[],
			partners: ~[]
		}
	}
	
	pub fn pair_with(@mut self, other: @mut Player) -> @Team {
		self.partners.push(other);
		other.partners.push(self);
		Team::new(self, other)
	}
	
	pub fn find_partner(@mut self, choices: &[@mut Player]) -> Option<@Team> {
		for choices.each() |part| {
			if !self.partners.contains(part) {
				return Some(self.pair_with(*part))
			}
		}
		None
	}
}

#[link_args = "src/gui/gui.o"]
extern {
	fn gui_main(argc: c_int, args: **c_char) -> c_int;
	fn gui_addPlayer(name: *c_char, gender: Gender, id: c_int, skill: c_int, wins: **Team);
	fn gui_removePlayer(id: c_int);
	fn gui_addWin(pid: c_int, oppid: c_int);
}

pub fn main() {
	unsafe {
		gui_main(os::rustrt::rust_get_argc(), os::rustrt::rust_get_argv());
	}
}
