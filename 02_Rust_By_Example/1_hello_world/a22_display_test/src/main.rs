use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, i) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", i)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
