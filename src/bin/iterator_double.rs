trait DoubleTrait<T: Iterator>: Iterator + Sized
where
    T::Item: Clone,
{
    fn double(self) -> Double<T>;
}
impl<T: Iterator> DoubleTrait<T> for T
where
    T::Item: Clone,
{
    fn double(mut self) -> Double<Self> {
        Double {
            item: self.next(),
            iter: self,
        }
    }
}

#[derive(Debug)]
struct Double<T: Iterator> {
    iter: T,
    item: Option<T::Item>,
}
impl<T: Iterator> Iterator for Double<T>
where
    T::Item: Clone + std::ops::Mul<Output = T::Item>,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        if let Some(n) = self.iter.next() {
            Some(n.clone() * n)
        } else {
            self.item = None;
            None
        }
    }
}

fn main() {
    println!("{:?}", (0..5).double().collect::<Vec<i32>>());
}
