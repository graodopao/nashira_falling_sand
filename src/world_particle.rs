
use raylib::prelude::*;

pub trait Particle {
    fn new() -> Self where Self: Sized;
}

#[derive(Clone)]
pub struct Liquid {
    pub color: Color,
}
impl Particle for Liquid {
    fn new() -> Self {
        Liquid { color: Color::BLUE }
    }
}

#[derive(Clone)]
pub struct Void {
    pub color: Color,
}

impl Particle for Void {
    fn new() -> Self {
        Void { color: Color::BLACK }
    }
}

// Sand
#[derive(Clone)]
pub struct Powder {
    pub color: Color,
}

impl Particle for Powder {
    fn new() -> Self { Powder { color: Color::YELLOW }
    }
}

#[derive(Clone)]
pub enum ParticleEnum {
    Void(Void),
    Liquid(Liquid),
    Powder(Powder),
}
