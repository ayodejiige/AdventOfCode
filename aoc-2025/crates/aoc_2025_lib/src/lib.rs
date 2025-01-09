pub(crate) mod common;

use paste::paste;

macro_rules! declare_modules {
    ($($mod_number:tt),*) => {
        $(
            paste! {
                pub mod [<day $mod_number>];
            }
        )*
    };
}

declare_modules!(1, 2, 3, 4, 5, 6, 7);