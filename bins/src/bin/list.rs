struct List<T: std::fmt::Display> {
    value: Option<T>,
    next: Option<Box<List<T>>>,
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            value: None,
            next: None,
        }
    }

    fn append(&mut self, value: T) {
        let next = List::new();
        let mut node = self;
        loop {
            if let Some(_) = &node.next {
                node = node.next.as_mut().unwrap();
                continue;
            } else {
                node.value = Some(value);
                node.next = Some(Box::new(next));
                break;
            }
        }
    }

    fn display(&self) {
        print!("[");
        let mut node = self;
        if let Some(value) = &node.value {
            print!("{}", value);
            node = &node.next.as_ref().unwrap();
        }
        loop {
            if let Some(value) = &node.value {
                print!(", {}", value);
                node = &node.next.as_ref().unwrap();
            } else {
                println!("]");
                break;
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list.display();
    list.append(3);
    list.display();
    list.append(5);
    list.display();
    list.append(10);
    list.display();
}
