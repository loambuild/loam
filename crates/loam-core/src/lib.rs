pub trait Lazy: Sized {
    fn get_lazy() -> Option<Self>;

    fn set_lazy(self);
}
