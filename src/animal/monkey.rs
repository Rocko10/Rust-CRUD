use Animal;

pub struct Monkey {
    name: String
}

impl Animal for Monkey {
    fn get_name(&self) -> &String {
        &self.name
    }
}
