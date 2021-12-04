mod myvec;
mod mystr;
mod myhash;

fn main() {
    let arr = vec![1, 1, 1, 1, 10];
    println!("mean {}, mode {}, median {}", myvec::mean(&arr), myvec::mode(&arr), myvec::median(&arr));
    assert_eq!(myvec::mean(&arr), 2.8);
    assert_eq!(myvec::mode(&arr), 1);
    assert_eq!(myvec::median(&arr), 1);

    let arr = vec![3, 0, 2, 1, 1, 3, 3, 0, 3];
    println!("mean {}, mode {}, median {}", myvec::mean(&arr), myvec::mode(&arr), myvec::median(&arr));
    assert_eq!(myvec::mean(&arr), 1.7777778);
    assert_eq!(myvec::mode(&arr), 3);
    assert_eq!(myvec::median(&arr), 2);

    let mystring = String::from("moreapple");
    println!("pig_latin {}", mystr::pig_latin(&mystring));
    assert_eq!(mystr::pig_latin(&mystring), "oreapple-may");

    let mystring = String::from("oogles");
    println!("pig_latin {}", mystr::pig_latin(&mystring));
    assert_eq!(mystr::pig_latin(&mystring), "oogles-hay");

    let mut mycompany = myhash::Company::new();
    loop {
        // TODO: Add Remove Employee
        println!("\n1. Add Employee");
        println!("2. Show Employees");
        println!("3. Quit");
        println!("Please select an action:");
    
        match myhash::read_input().trim().parse() {
            Ok(num) => {
                println!();
                match num {
                    1 => mycompany.add_employee(),
                    2 => mycompany.show_employees(),
                    3 => break,
                    _ => println!("ERROR: Unknown selection"),
                }
            }
            Err(_) => continue,
        };
    }
}
