pub trait Log<T>
where
    T: ToOwned
{
    fn log(&mut self, value: &T);
}
