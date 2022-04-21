struct Odd {
    number: isize,
    max: isize,
}

impl Iterator for Odd {
    type Item = isize;



    fn next(&mut self) -> Option<Self::Item> {
        self.number +=2;
        if self.number <= self.max {
            Some(self.number)
        } else {
            None
        }
    }
}

impl Odd {
    fn new(max: isize) -> Self {
        Self {number: 1, max}
    }
}

fn test() {
    let mut n = Odd::new(9);
    println!("{}", n.number);
    n.next();
    println!("{}", n.number);
    for o in n {
        println!("odd: {}", o);
    }
}

struct Friends { 
    names: Vec<String>,
}

impl<'a> IntoIterator for &'a mut Friends {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}
fn main() {
    

}