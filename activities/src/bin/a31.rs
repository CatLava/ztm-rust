// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

#[derive(Debug)]
struct Material {
    name: String,
    size_m2: f32,
    price: f32, 
}


trait Cost {
    fn cost(&self) -> f32;
}

impl Cost for Material {
    fn cost(&self) -> f32{
        self.size_m2*self.price
    }
}

fn total_cost(list: &Vec<Box<dyn Cost>>) -> f32 {
    list.iter().map(|item| item.cost()).sum() 
}
fn main() {
    let wood = Material {
        name: "wood".to_string(),
        size_m2: 31.0,
        price: 10.0,
    };
    println!("{:?}", wood.cost());

    let tile = Material {
        name: "tile".to_string(),
        size_m2: 25.0,
        price: 21.0,
    };


    let steel = Material {
        name: "steel".to_string(),
        size_m2: 125.0,
        price: 8.0,
    };

    println!("{:?}", tile.price);

    let total: Vec<Box<dyn Cost>> = vec![Box::new(tile), Box::new(wood), Box::new(steel)];
    //println!("{:?}", total);
    println!("Total bill of materials: ${}", total_cost(&total));
}
