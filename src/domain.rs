use std::collections::HashSet;

pub struct SimpleDomain{
    set: HashSet<i16>,
}

impl SimpleDomain{

    pub fn new(start: i16 , end: i16) -> SimpleDomain {
        let mut s = HashSet::new();
        for i in start..end{
            s.insert(i);
        }
        SimpleDomain{set: s}
    }

    pub fn getPosition(self) -> i16{
        1
    }
}