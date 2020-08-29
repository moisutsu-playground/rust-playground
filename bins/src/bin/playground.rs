trait Joins {
    fn joins(&self, sep: &str) -> String;
}

impl<T: std::string::ToString> Joins for Vec<T> {
    fn joins(&self, sep: &str) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep).to_string()
    }
}

macro_rules! echo {
    ($ x : expr ) => {
        println!("{}", $x);
    };
}

fn main() {
    let v = vec![3, 1, 9, 1];
    echo!(v.joins(" "));
}
