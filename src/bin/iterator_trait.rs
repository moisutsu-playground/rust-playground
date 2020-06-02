trait WithPrevTrait<T: Iterator>: Iterator + Sized
where
    T::Item: Clone,
{
    fn with_prev(self) -> WithPrev<T>;
}
impl<T: Iterator> WithPrevTrait<T> for T
where
    T::Item: Clone,
{
    fn with_prev(mut self) -> WithPrev<Self> {
        WithPrev {
            prev: self.next(),
            iter: self,
        }
    }
}

#[derive(Debug)]
struct WithPrev<T: Iterator> {
    iter: T,
    prev: Option<T::Item>,
}
impl<T: Iterator> Iterator for WithPrev<T>
where
    T::Item: Clone,
{
    type Item = (T::Item, T::Item);
    fn next(&mut self) -> Option<(T::Item, T::Item)> {
        if let Some(n) = self.iter.next() {
            if let Some(p) = self.prev.replace(n.clone()) {
                Some((p, n))
            } else {
                None
            }
        } else {
            self.prev = None;
            None
        }
    }
}

fn main() {
    println!("{:?}", (0..5).with_prev().collect::<Vec<(i32, i32)>>());
}
