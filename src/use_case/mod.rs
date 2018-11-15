use animal::Animal;

mod add;
mod get;

pub trait Add {
    fn execute(&mut self, animal: Box<impl Animal + 'static>);
}

pub trait Get {
    fn execute(&self) -> &Vec< Box<dyn Animal> >;
}
