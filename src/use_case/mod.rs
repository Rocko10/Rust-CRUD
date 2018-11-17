use animal::Animal;

mod add;
mod get;
mod show;

pub trait Add {
    fn execute(&mut self, animal: Box<impl Animal + 'static>);
}

pub trait Get {
    fn execute(&self) -> &Vec< Box<dyn Animal> >;
}

pub trait Show {
    fn execute(&self, name: String) -> Option<&Box<dyn Animal>>;
}
