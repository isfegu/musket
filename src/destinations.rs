pub trait Destination {
    fn shot(&self);
}

pub struct LinkedIn;
pub struct X;

impl Destination for LinkedIn {
    fn shot(&self) {
        println!("LinkedIn")
    }
}

impl Destination for X {
    fn shot(&self) {
        println!("X")
    }
}
