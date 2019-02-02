struct ThrustGame {
	thrusters: Vec<String>,
	thrustees: Vec<String>,
}

impl Default for ThrustGame {
	fn default() -> ThrustGame {
		ThrustGame{thrusters: Vec::new(), thrustees: Vec::new()}
	}
}

impl ThrustGame {
	fn add_thruster(&mut self, thruster: & std::string::String) {
		self.thrusters.push(thruster.to_string());
	}
}

impl ThrustGame {
	fn add_thrustee(&mut self, thrustee: & std::string::String) {
		self.thrustees.push(thrustee.to_string());
	}
}

impl ThrustGame {
	fn sort_thrusters(&mut self) {
		self.thrusters.sort();
	}
}

impl ThrustGame {
	fn sort_thrustees(&mut self) {
		self.thrustees.sort();
	}
}

fn main() {
	let test_thrusters = vec![String::from("Homosexuality"), String::from("Degeneracy"), String::from("Moefag"), String::from("fuckoff")];
	let test_thrustees  = vec![String::from("Gay is ____"), String::from("Kenny is ____"), String::from("Fuck you _____")];
	let thruster = String::from("Your Gay");
	let thrustee = String::from("The reason Kenny is gay is ________.");

	let mut instance = ThrustGame::default();

	let mut test = ThrustGame{thrusters: test_thrusters, thrustees: test_thrustees};

	test.sort_thrusters();

	for x in &test.thrusters {
		println!("{}", x);
	}

	test.sort_thrustees();

	for x in &test.thrustees {
		println!("{}", x);
	}
}







// mod thrust_shit {
// 	pub struct Thrust {
// 		pub thruster: i32,
// 		pub shit: i32
// 	}

// 	impl Thrust {
// 		pub fn new() -> Thrust {
// 			Thrust {
// 				thruster: 69,
// 				shit: 12
// 			}
// 		}
// 	}

// 	impl Thrust {
// 		pub fn change_thrust(&mut self) {
// 			self.thruster = 727;
// 		}
// 	}
// }

// struct Test {
// 	thruster: i32
// }

// impl Test {
// 	fn new() -> Test {
// 		Test {
// 			thruster: 69
// 		}
// 	}
// }

// impl Test {
// 	fn change_thrust(&mut self) {
// 		self.thruster = 727;
// 	}
// }
 
 
// fn main() {
// 	let mut thrust_instance = thrust_shit::Thrust{thruster: 420, shit: 69};
// 	let t = Test {
// 		thruster: 32
// 	};
// 	println!("{}", thrust_instance.thruster);
// 	thrust_instance.change_thrust();
// 	println!("{}", thrust_instance.thruster);
// }