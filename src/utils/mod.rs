pub mod log;
pub use log::*;

pub trait Unzip<T1, T2> {
    fn unzip(self) -> (Vec<T1>, Vec<T2>);
}

impl<T1, T2, I> Unzip<T1, T2> for I
where
    I: IntoIterator<Item = (T1, T2)>,
{
    fn unzip(self) -> (Vec<T1>, Vec<T2>) {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();

        for (item1, item2) in self {
            vec1.push(item1);
            vec2.push(item2);
        }

        (vec1, vec2)
    }
}
