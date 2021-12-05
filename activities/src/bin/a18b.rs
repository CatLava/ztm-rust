// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeDepartment {
    Maintenance,
    Marketing,
    Managers,
    LineStaff,
    Kitchen,
    AssemblyTech,
}

struct EmployeeStatus {
    department: EmployeeDepartment,
    // this could reference an enum versus this set up clear

    active: bool,
}

fn PermitEntry(employee: EmployeeStatus) -> Result<bool, String> {
    // a match could be used here instead of an if
    // this depends on the s
    if employee.active == true {
        match employee.department {
            EmployeeDepartment::Managers => Ok(true),
            _ => Err("Not in department".to_string()),
        }
    } else {
        Err("Employee not active".to_string())
    }
}
fn main() {

    fn check_employee(emp: EmployeeStatus) -> Result<bool, String> {
        let juan = PermitEntry(emp)?;
        return Ok(juan)
    }

    let p1 = check_employee(EmployeeStatus {
        department: EmployeeDepartment::Managers,
        active: true,
        }
    );

    let p2 = check_employee(EmployeeStatus {
        department: EmployeeDepartment::Managers,
        active: false,
        }
    );

    let p3 = check_employee(EmployeeStatus {
        department: EmployeeDepartment::Kitchen,
        active: true,
        }
    );

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3)
}
