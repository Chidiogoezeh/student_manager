struct StudentId(u32);
struct Student {
    id: StudentId,
    name: String,
    age: u8,
    course: String,
    score: f64,
}

enum MenuOption {
    Add,
    Delete,
    List,
    Exit,
}
fn main() {
    println!("Student Management System");
}
