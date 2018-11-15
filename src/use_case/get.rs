use gateway::Gateway;
use use_case::Get;
use animal::Animal;

pub struct GetImp<'a> {
    data_provider: &'a dyn Gateway
}

impl<'a> Get for GetImp<'a> {
    fn execute(&self) -> &Vec< Box<dyn Animal> > {
        self.data_provider.get_animals()
    }
}

impl<'a> GetImp<'a> {
    pub fn new(data_provider: &'a dyn Gateway) -> Self {
        GetImp { data_provider }
    }
}

#[cfg(test)]
mod test {

    use ::memory::Memory;
    use super::GetImp;
    use ::use_case::Get;
    use ::use_case::add::AddImp;
    use ::use_case::Add;
    use ::animal::duck::Duck;

    #[test]
    fn test_get_and_add() {
        let mut mem = Memory::new();
        let d1 = Duck::new("Daffy".to_string());
        let d2 = Duck::new("Donald".to_string());

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(d1));
            adder.execute(Box::new(d2));
        }
        {
            let getter = GetImp::new(&mem);
            assert_eq!(getter.execute().len(), 1);
        }
    }
}
