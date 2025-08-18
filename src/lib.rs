#[macro_export]
macro_rules! ternary {
    ($condition:expr, $true_val:expr, $false_val:expr) => {
        if $condition { $true_val } else { $false_val }
    };
}

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