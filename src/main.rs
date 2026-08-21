use std::io;

struct StudentId(u32);
struct Student {
    id: StudentId,
    name: String,
    age: u8,
    course: String,
    score: f64,
}

impl Student {
    fn new(id: StudentId, name: String, age: u8, course: String, score: f64) -> Student {
        Student {
            id,
            name,
            age,
            course,
            score,
        }
    }
}

enum MenuOption {
    Add,
    Delete,
    List,
    Exit,
}

fn show_menu() {
    println!();
    println!("== Student Management System ==");
    println!("1. Add students");
    println!("2. Delete students");
    println!("3. List students");
    println!("4. Exit");
    println!("=======================");
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn get_menu_option(choice: u32) -> MenuOption {
    match choice {
        1 => MenuOption::Add,
        2 => MenuOption::Delete,
        3 => MenuOption::List,
        4 => MenuOption::Exit,
        _ => MenuOption::Exit,
    }
}
fn main() {
    show_menu();

    println!("Enter your choice");

    let input = read_input();

    let choice: u32 = input.parse().expect("Please enter a valid number");

    let option = get_menu_option(choice);

    match option {
        MenuOption::Add => {
            let student = Student::new(
                StudentId(1),
                String::from("Chidiogo"),
                30,
                String::from("Backend Development"),
                80.6,
            );

            println!("Student created");
            println!("Name: {}", student.name);
        }

        MenuOption::Delete => {
            println!("Deleting student...");
        }

        MenuOption::List => {
            println!("Listing students...");
        }

        MenuOption::Exit => {
            println!("Exiting...");
        }
    }
}
