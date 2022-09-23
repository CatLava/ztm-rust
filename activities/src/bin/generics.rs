trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}
struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as pilot");
    }
    fn process(&self) {
        println!("pilot to cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("checked in as passenger");
    }
    fn process(&self) {
        println!("takes a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("checked in as cargo");
    }
    fn process(&self) {
        println!("moved to storage");
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}
fn main() {
    let paul = Passenger;
    let katy = Pilot;
    let cargo1 = Cargo;
    process_item(paul);
    process_item(katy);
    process_item(cargo1);
}