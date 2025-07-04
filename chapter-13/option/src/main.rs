
mod radx64
{
    #[derive(Debug)]
    pub enum Option<T>
    {
        None,
        Some(T),
    }

    impl<T> Option<T>
    {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where 
            F: FnOnce() -> T
        {
            match self {
                self::Option::Some(x) => x,
                self::Option::None => f(),
            }
        }
    }
} 

fn main() {

    let some = radx64::Option::Some(1); 
    let none : radx64::Option<i32> = radx64::Option::None;

    println!("{some:?} and {none:?}");

    println!("Current value of none is {}", none.unwrap_or_else(|| {32}));
}
