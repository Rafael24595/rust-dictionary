pub struct ComboPermuter {
    combo: String,
    min: i8
}

impl ComboPermuter {

    pub fn new(combo: String, min: Option<i8>) -> ComboPermuter {
        let mut min_base = combo.len() as i8;
        if min.is_some() && min.unwrap() > 0 {
            min_base = min.unwrap();
        }
        return ComboPermuter {
            combo: combo,
            min: min_base
        }
    }

    pub fn permute(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.build_permutations(&mut result, &mut Vec::new());
        return result; 
    }

    fn build_permutations(&self, result: &mut Vec<String>, current: &mut Vec<usize>) {
        if (current.len() as i8) >= self.min {
            let permutation = self.build_current(current);
            result.push(permutation);
        }

        if current.len() == self.combo.len() {
            return;
        }
    
        for (i, _) in self.combo.chars().enumerate() {
            if current.iter().position(|&ci| ci == i).is_some() {
                continue;
            }
    
            current.push(i);
    
            self.build_permutations(result, current);
    
            current.pop();
        }
    }

    fn build_current(&self, result: &mut Vec<usize>) -> String {
        let mut combo = String::new();
        for position in result {
            let b: u8 = self.combo.as_bytes()[*position];
            let c: char = b as char;
            combo.push(c);
        }
        return combo;
    }

}