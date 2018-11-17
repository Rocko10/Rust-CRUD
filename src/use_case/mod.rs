use animal::Animal;

mod add;
mod get;
mod show;
mod delete;

pub trait Add {
    fn execute(&mut self, animal: Box<impl Animal + 'static>);
}

pub trait Get {
    fn execute(&self) -> &Vec< Box<dyn Animal> >;
}

pub trait Show {
    fn execute(&self, name: String) -> Option<&Box<dyn Animal>>;
}

pub trait Delete {
    fn execute(&mut self, name: String) -> Result<Box<dyn Animal>, &'static str>;
}
