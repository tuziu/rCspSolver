use crate::SwapContainer::SwapContainer;

type Ty = i64;
pub struct Variable{
    desc: String,
    domain: SwapContainer,
}


impl Variable{
    pub fn getDomain(&self) -> Vec<Ty>{
        Vec::new()
    }

    pub fn remove(&mut self, v: Ty) -> usize {
        1
    }
}
