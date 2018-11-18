mod animal;
use animal::Animal;

mod memory;
mod gateway;

mod use_case;
mod presenter;

mod response;

use memory::Memory;
use animal::duck::Duck;
use animal::monkey::Monkey;
use use_case::Add;
use use_case::add::AddImp;
use use_case::Get;
use use_case::get::GetImp;
use presenter::cli::Cli;

fn main() {

    let mut mem = Memory::new();
    let cli = Cli::new();

    let daffy = Duck::new("Daffy".to_string());
    let donald = Duck::new("Donald".to_string());
    let mojo = Monkey::new("Mojo".to_string());

    {
        let mut adder = AddImp::new(&mut mem);
        adder.execute(Box::new(daffy));
        adder.execute(Box::new(donald));
        adder.execute(Box::new(mojo));
    }

    let getter = GetImp::new(&mem, &cli);
    getter.execute();


}
