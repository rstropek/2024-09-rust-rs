mod pets {
    pub struct Pet {
        name: String,
        age: u8,
        species: String,
    }

    pub fn print_pet_data(pet: &Pet) {
        println!("{} is a {} year old {}", pet.name, pet.get_age(), pet.get_species());
    }

    impl Pet {
        pub fn new(name: String, age: u8, species: String) -> Pet {
            Pet { name, age, species }
        }

        #[allow(dead_code)]
        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn get_species(&self) -> &str {
            &self.species
        }
    }
}

fn main() {
    let pet = pets::Pet::new("Fluffy".to_string(), 3, "Dog".to_string());

    pets::print_pet_data(&pet);
}
