
pub struct User {
    name: String,
    age: i32,
}

impl User {
    pub fn new(name: String, age: i32) -> User {
        User {
            name,
            age,
        }
    }

    pub fn get_name(self) -> String {
        self.name
    }
}
