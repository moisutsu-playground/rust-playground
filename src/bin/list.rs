#[derive(Copy)]
struct List {
    value: i32,
    next: Option<Box<List>>,
}

impl List {
    fn new(value: i32) -> List {
        List {
            value: value, next: None
        }
    }

    fn append(&mut self, value: i32) {
        let mut head = self;
        while head.next.is_some() {
            head = &mut head.next.unwrap();
        }
    }
}

fn main() {
    let mut ls = List::new(5);
}
