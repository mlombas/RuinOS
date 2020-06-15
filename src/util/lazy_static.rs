pub struct LazyStatic<T>{
    value: &'static mut Option<T>,
}
