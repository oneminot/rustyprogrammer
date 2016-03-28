struct Philosopher {
    name: String, 
	year: i16,
}

impl Philosopher {
    fn new(name: &str, year: i16) -> Philosopher {
        Philosopher {
            name: name.to_string(),
			year: year as i16,
        }
    }

	fn eat(&self){
		println!("{} of {} is done eating.", self.name, self.year);
	}
}

fn main() {
    let philosophers = vec![
	Philosopher::new("Judith Butler", 1956),
    Philosopher::new("Gilles Deleuze", 1925),
    Philosopher::new("Karl Marx", 1818),	
	Philosopher::new("Emma Goldman", 1869), 
	Philosopher::new("Michel Foucault", 1926),
	];
	for p in philosophers {
		p.eat();
	}
}