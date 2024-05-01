fn drowned_message() {
    println!("The plant is drowned!");
}

#[derive(Debug)]
pub enum PlantType {
    Sunflower,
    Rose,
    Tulip,
}

pub mod plant {
    #[derive(Debug)]
    pub struct Plant {
        pub name: super::PlantType,
        pub age: u32,
        pub water_level: u32,
        pub is_drowned: bool,
    }

    impl Plant {
        pub fn new(name: super::PlantType, age: u32) -> Plant {
            Plant {
                name,
                age,
                water_level: 0,
                is_drowned: false,
            }
        }

        // private function
        fn kill(&mut self) {
            self.is_drowned = true;
            super::drowned_message();
        }

        pub fn grow(&mut self) {
            self.age += 1;
        }

        pub fn water_plant(&mut self) {
            if self.is_drowned {
                super::drowned_message();
                return;
            }
            println!("Watering the plant");
            self.water_level += 1;
            if self.water_level >= 10 {
                self.kill();
            }
        }
    }
}
