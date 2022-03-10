use crate::Model;

pub trait Test {
    fn test(&self);
}

impl Test for Model {
    fn test(&self) {
        println!("a");
    }
}
