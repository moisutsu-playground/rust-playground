// まずはイテレータに生やすメソッドを定義し、自作したイテレータの働きをする構造体を返す
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
    // このメソッドは、自作した構造体を返す働きしかない
    fn double(self) -> Double<Self> {
        Double {
            iter: self,
        }
    }
}

// イテレータの役割をする自作の構造体
// この構造体に生やすnextがイテレータの振る舞いを決める
#[derive(Debug)]
struct Double<T: Iterator> {
    iter: T,
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
            None
        }
    }
}

fn main() {
    println!("{:?}", (0..5).double().collect::<Vec<i32>>());
}
