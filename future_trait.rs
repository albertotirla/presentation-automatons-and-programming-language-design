pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
pub enum Poll<T> {
    Ready(T),
    Pending,
}