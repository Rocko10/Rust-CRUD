mod monkey;

pub trait Animal {
    fn get_name(&self) -> &String;
}
