use std::borrow::BorrowMut;
// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

// This is an incorrect setup below
// ideally want to create a vec with two vehicles
// Rc new
// Rc clone for the corp and management

use std::rc::Rc;
use std::cell::RefCell;

type Manage = Rc<RefCell<Vec<Vehicle>>>;
struct Corporate(Manage);

struct StoreFront(Manage);

#[derive(Debug)]
enum VehicleType {
    Sedan,
    SUV,
    Coup,
}
#[derive(Debug)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Vehicle {
    vehicle_type: VehicleType,
    status: VehicleStatus,
    vin: String
}

impl Vehicle {
    fn new(vehicle: Vehicle) -> Vehicle {
        vehicle
    }

    fn update_status(&mut self, new_status: VehicleStatus) {
        self.status = new_status;

    }
}

fn main() {
    let rental1 = Rc::new(RefCell::new(Vehicle::new( Vehicle {
        vehicle_type: VehicleType::Sedan,
        status: VehicleStatus::Available,
        vin: "123456".to_string(),
    })));

    let rental2 = Rc::new(RefCell::new(Vehicle::new( Vehicle {
        vehicle_type: VehicleType::SUV,
        status: VehicleStatus::Unavailable,
        vin: "583590".to_string(),
    })));

    println!("{:?}", rental1);

    println!("{:?}", rental2);
    {
        // need to use borrow mut in order to access update_status
        rental2.borrow_mut().update_status(VehicleStatus::Rented)
    }
    println!("{:?}", rental2);

    let corp = Corporate(Rc::clone(&rental1));
    {
        corp.borrow_mut().update_status(VehicleStatus::Maintenance);
    }
    dbg!(corp.0.borrow());


}
