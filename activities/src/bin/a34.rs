// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state


struct Luggage<State>{
    id: u32,
    state: State,
}

//impl a scenario for a state transfer
impl<State> Luggage<State> {
    fn transition<NewState> (self, state: NewState) -> Luggage<NewState> {
        Luggage { 
            id: self.id,
             state: state }
    }

}
impl Luggage<CheckIn> {
    fn new(id: u32) -> Self {
        Self {
            id: id,
            state: CheckIn
        }
    }

    fn onload(self, flightNumber: i32) -> Luggage<OnLoading> {
        println!("onboarded to flight: 9420");
        self.transition(OnLoading {flightNumber})
    }
}
   
        
impl Luggage<OnLoading> {
    fn offload(self, terminal: i32) -> Luggage<OffLoading> {
        println!("pickup luggage at terminal {}", terminal);

        self.transition(OffLoading {terminal})
    }
}

impl Luggage<OffLoading> {
    fn pickup(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}

struct CheckIn;
struct OnLoading {
    flightNumber: i32
}
struct OffLoading {
    terminal: i32
}

struct AwaitingPickup;

fn main() {
    let test = Luggage::new(9);
    let nextstate = test.onload(9420).offload(8);
}
