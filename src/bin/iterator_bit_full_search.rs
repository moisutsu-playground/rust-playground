trait BitFullSearchTrait<T: Iterator>: Iterator + Sized
where
    T::Item: Clone,
{
    fn bit_full_search(self) -> BitFullSearch<T>;
}
impl<T: Iterator> BitFullSearchTrait<T> for T
where
    T::Item: Clone,
{
    fn bit_full_search(self) -> BitFullSearch<Self> {
        BitFullSearch {
            iter: self,
            bit: 0,
            length: 0,
            v: vec![]
        }
    }
}

#[derive(Debug)]
struct BitFullSearch<T: Iterator> {
    iter: T,
    bit: usize,
    length: usize,
    v: Vec<T>
}
impl<T: Iterator> Iterator for BitFullSearch<T>
where
    T::Item: Clone + std::ops::Mul<Output = T::Item>,
{
    // Item が返り値
    type Item = Vec<T::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            self.length = self.count();
        }
        if self.v.is_empty() {
            self.v = self.collect();
        }
        while self.bit < 1 << self.length {
            let mut tmp_v = vec![];
            for i in 0..self.length {
                if self.bit & (1 << i) == 0 {
                    continue;
                }
                tmp_v.push()
            }
        }
        // if let Some(n) = self.iter.next() {
        //     Some(n.clone() * n)
        // } else {
        //     None
        // }
        None
    }
}

fn main() {
    println!("{:?}", (0..5).bit_full_search().collect::<Vec<i32>>());
}
