use animal::Animal;

pub trait Gateway {
    fn add(&mut self, animal: Box<dyn Animal>);
    fn get_animals(&self) -> &Vec< Box<dyn Animal> >;
    // TODO: show, delete
}
