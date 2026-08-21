struct StudentId(u32);
struct Student {
    id: StudentId,
    name: String,
    age: u8,
    course: String,
    score: f64,
}
fn main() {
    let mut student = Student {
        id: StudentId(1),
        name: String::from("Chidiogo"),
        age: 30,
        course: String::from("Backend Development"),
        score: 85.6,
    };

    student.score = 90.4;

    println!("Student ID: {}", student.id.0);
    println!("Student Name: {}", student.name);
    println!("Student Age: {}", student.age);
    println!("Student Course: {}", student.course);
    println!("Student Score: {}", student.score);
}
