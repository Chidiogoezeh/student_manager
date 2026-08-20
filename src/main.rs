struct Student {
    id: u32,
    name: String,
    age: u8,
    course: String,
    score: f64,
}
fn main() {
    let student = Student {
        id: 1,
        name: String::from("Chidiogo"),
        age: 30,
        course: String::from("Backend Development"),
        score: 85.6,
    };

    println!("Student: {}", student.name);
}
