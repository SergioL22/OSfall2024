pub struct Student {
    name:String,
    major: String,

}

pub impl Student {
    pub fn introduce(&mut self) {
        println!("Hello, my name is {} and my major is {}", self.name,self.major)
    }
}

pub let mut student_1 = Student{
    name: String::from("Sergio"),
    major : String::from("CS"),
};

student_1.introduce(); 

mod test {
    use super::*;

    fn test_student_creation(){
        let s = Student::new_student_("Alex".to_String(), "Computer Science".to_String());
        assert_eq!(s.name, "Alex".to_String());
    }
}