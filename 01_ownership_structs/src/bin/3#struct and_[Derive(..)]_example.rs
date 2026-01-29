#[derive(Debug, Clone)]
struct ROleColor(i32, i32, i32);

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    id: u64,
    department: String,
    active: bool,
    badge_color: ROleColor,
}

fn main() {
    let blue_color = ROleColor(1, 1, 1);
    let black = ROleColor(0, 0, 0);

    let mut emp1 = Employee {
        name: String::from("adnan"),

        id : 11,

        department: String::from("brj al7lam"),

        active: true,
        badge_color: black,
    };

    emp1.badge_color = blue_color;

    print_employee_card(&emp1);


    let emp2 = Employee{

        name : String :: from("ahmed"),

        id : 12,

        active: false,
        ..emp1.clone()
    };

    println!("");
print_employee_card(&emp2);


}


fn print_employee_card(emp: &Employee) {
    println!("BADGE ID: {}", emp.id);
    println!(" Name:     {}", emp.name);
    println!("Dept:     {}", emp.department);
    println!("Color:    {:?}", emp.badge_color);
    if emp.active {
        println!("Status:   Active");
    } else {
        println!("Status:   Inactive");
    }
}