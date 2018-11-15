use animal::Animal;

mod add;

pub trait Add {
    fn execute(&mut self, animal: Box<impl Animal + 'static>);
}
