pub struct WindowCount<I> {
    pub(crate) iter: I,
    count: u32,
}

// IteratorExt 「继承」Iterator，这样可以使用 Iterator 的全部功能
pub trait IteratorExt: Iterator {
    fn window_count(self, count: u32) -> WindowCount<Self>
    where
        Self: Sized,
    {
        WindowCount { iter: self, count }
    }
}

// 让所有实现了 Iterator 的 T 都自动实现 IteratorExt
impl<T: ?Sized> IteratorExt for T where T: Iterator {}

impl<I: Iterator> Iterator for WindowCount<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = (0..self.count)
            .filter_map(|_| self.iter.next())
            .collect::<Vec<_>>();
        if data.is_empty() {
            return None;
        } else {
            return Some(data);
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = data.iter().window_count(3).collect::<Vec<Vec<_>>>();
    // [[1, 2, 3], [4, 5]]
    println!("{:?}", result);
}
