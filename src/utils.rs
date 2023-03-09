pub mod prelude{
    pub trait PushAndReturn<T>{
        fn push_and_return(self, val: T) -> Self;
    }

    impl<T> PushAndReturn<T> for Vec<T>{
        fn push_and_return(mut self, val: T) -> Self{
            self.push(val);
            self
        }
    }
}
