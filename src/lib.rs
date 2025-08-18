

///A ternary operator implementation for rust
/// 
/// ```
/// let condition = false;
/// 
/// let result = ternary!(condition, "val if condition is true", "val if condition is false")
/// assert_eq!(result, "val if condition is false");
/// ```
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $true_val:expr, $false_val:expr) => {
        if $condition { $true_val } else { $false_val }
    };
}

///A macro wrapper for std::dbg! which is executed only in non release builds
#[macro_export]
macro_rules! debug {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        debug!(ternary!(true, "foo", "boo"));
    }
}