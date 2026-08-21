struct Student {
    id: u32,
    name: String,
    age: u8,
    course: String,
    score: f64,
}
fn main() {
    let mut student = Student {
        id: 1,
        name: String::from("Chidiogo"),
        age: 30,
        course: String::from("Backend Development"),
        score: 85.6,
    };

    student.score = 90.4;
    println!("Student: {}", student.score);

    let age = student.age;
    println!("Age: {}", age);

    let age = age + 1;
    println!("New age: {}", age);

    let student_id = "1004";
    let student_id = student_id.parse::<u32>().unwrap();

    println!("Passed Student ID (u32): {}", student_id);

    // Mutability
    let mut score = 75;
    score = 86;
    println!("Mutable score: {}", score);

    // Shadowing
    let score = 75;
    let score = 86;
    println!("Shadowed score: {}", score);
}
