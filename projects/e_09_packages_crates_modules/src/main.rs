use e_09_packages_crates_modules::{plant::Plant, PlantType};
use rand::Rng;

fn main() {
    let mut plant = Plant::new(PlantType::Sunflower, 0);

    println!("Plant name: {:?}\n", plant.name);
    for i in 0..11 {
        if i % 2 == 0 {
            plant.grow();
        }
        plant.water_plant();
        println!(
            "Age: {}\nWater Level: {}\nIs Drowned: {}\n",
            plant.age, plant.water_level, plant.is_drowned
        );
    }

    let plant_type = match rand::thread_rng().gen_range(0..3) {
        0 => PlantType::Sunflower,
        1 => PlantType::Rose,
        _ => PlantType::Tulip,
    };

    print!("Random plant type: {:?}\n", plant_type)
}
