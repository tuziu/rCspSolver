use crate::domain::SimpleDomain;

pub enum Constraint{
    Unary,
    Binary,
}

impl Constraint{
    fn isUnary(&self)-> bool{
        match self{
            Constraint::Unary => true,
            _ => false,
        }
    }
    fn isBinary(&self)-> bool{
        match self{
            Constraint::Binary => true,
            _ => false,
        }
    }
}

pub trait Unary{
    fn test(self,domain: SimpleDomain) -> bool;
}

pub trait Binary{
    fn test(self,left: SimpleDomain, right: SimpleDomain) -> bool;
}

