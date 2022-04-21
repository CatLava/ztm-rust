use std::collections::HashMap;

//Eq and Partial Eq allows using this for key in hashmap
#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

// to iterate over this need the into iter trait
struct FruitStand {
    fruit: HashMap<Fruit, u32>
}

// standard into iter function go go over
impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.into_iter()
    }
}

// next is borrow this similar function
// introduce lifetimes
impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit,&'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter()
    }
}

impl<'a> IntoIterator for &'a mut FruitStand {
    // we want to change the values and not the keys
    type Item = (&'a Fruit,&'a mut u32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter_mut()
    }
}

//mutable borrow as well 
fn main() {
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Apple, 2);
    fruit.insert(Fruit::Orange, 8);

    let fruit = fruit;

    let mut store = FruitStand {fruit};

    // for .. in loop will call into_iter
    for (fruit, stock) in &store {
        println!("{:?} : {:?}", fruit, stock)
    }

    for (fruit, stock) in &mut store {
        *stock += 10;
        println!("{:?} : {:?}", fruit, stock)
    }
}