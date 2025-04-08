//In class Assignment
 
//Create a struct student (major)
struct Student{
    major:String,
}
//Higher order functions update majors
fn update_major(collection: &mut [Student],behavior:fn(&mut Student, String)) {
    for student in collection.iter_mut() {
        behavior(student, "Computer Science".to_string());
    }
}
//First Order functions, assign_major(student, major_declared)
fn assign_major(s: &mut Student, major:String){
    s.major = major;   
}

fn print_students(students: &[Student]) {
    println!("Major: ");
    for (i,student) in students.iter().enumerate(){
        println!("Student {} major {}", i+1, student.major)
    }
}
//Create a vector for students1,2,3 and update all students major
fn main(){
    let mut students = vec![
        Student {major: ("Biology").to_string()},
        Student {major: ("Education").to_string()},
        Student {major: ("Engineering").to_string(),}
    ];

    print!("Initial: ");
    print_students(&students);
    
    update_major(&mut students, assign_major);

    print!("Updated: ");
    print_students(&students);
    
}