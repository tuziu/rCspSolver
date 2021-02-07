use crate::constraint::Binary;
use crate::constraint::Unary;
use crate::domain::SimpleDomain;
use crate::variable::Variable;
use std::collections::VecDeque;

type Value = i64;
struct ValueWarper {
    value: Value,
    pos: i32,
}

struct Oblique {}

// impl Binary for Oblique{
//     fn test(self, left: ValueWarper, right: ValueWarper) -> bool{
//         isOblique((left.pos,left.value),(right.pos,right.value))
//     }
// }

fn isOblique(left: (i16, i16), right: (i16, i16)) -> bool {
    let a = (left.0 - right.0).abs();
    let b = (left.1 - right.1).abs();
    a != b
}

fn isHorizontal(left: (i16, i16), right: (i16, i16)) -> bool {
    (left.0 != right.0) && (left.1 == right.1)
}

struct Mill {
    domains: Vec<SimpleDomain>,
    variables: Vec<Variable>,
    // binaryConstraints: Vec<Binary>,
}

fn generate(qeue: &mut VecDeque<(usize, usize)>, from: usize, to: usize) {
    for i in from..to {
        qeue.push_back((from, i));
        qeue.push_back((i, from));
    }
}

impl Mill {
    // fn grind() -> Vec<i16> {

    // }

    // fn addBinaryConstraint(&mut self,constraint: Binary){
    //     // self.push(constraint);
    // }

    fn addVariable(&mut self, variable: Variable) {
        // self.push(variable);
    }

    fn assign() {}

    fn checkConstraint(i: Value, j: Value) -> bool {
        true
    }

    fn revise(&self, vi: usize, vj: usize) -> bool {
        let mut di = self.variables[vi];
        let mut dj = self.variables[vj];
        let mut toDelete = Vec::new();
        for i in di.getDomain() {
            if dj.getDomain().iter().all(|j| !Mill::checkConstraint(i, *j)) {
                toDelete.push(i);
            }
        }
        if !toDelete.is_empty() {
            toDelete.iter().for_each(|v| {
                di.remove(*v);
            });
            return true;
        }
        false
    }

    fn arcConsistency(&self, pos: usize) -> bool {
        let mut q = VecDeque::new();
        generate(&mut q, pos, self.variables.len());
        while let Some((vk, vm)) = q.pop_front() {
            if self.revise(vk, vm) {
                if self.variables[vk].getDomain().is_empty() {
                    return false;
                } else {
                    generate(&mut q, vk, self.variables.len());
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::isOblique;

    #[test]
    fn isOblique_1() {
        let d = (0, 0);

        assert!(!isOblique(d, (1, 1)));
        assert!(!isOblique(d, (1, -1)));
        assert!(!isOblique(d, (-1, 1)));
        assert!(!isOblique(d, (-1, -1)));
    }
}
