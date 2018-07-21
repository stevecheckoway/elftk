macro_rules! declare_constant {
    ($id: ident : $t:ty = ($val:expr, $display:expr), $seealso:expr) => {
        #[doc = $display]
        #[doc = $seealso]
        pub const $id: $t = $val;
        //declare_constant! { #[doc = $display] $id: $t = ($val, $display), $seealso }
    };
    ($lo:ident, $hi:ident : $t:ty = ($lo_val:expr, $hi_val:expr, $display:expr), $seealso:expr) => {
        declare_constant! { #[doc = $display] $lo, #[doc = $display] $hi : $t = ($lo_val, $hi_val, $display), $seealso }
    };
    ($(#[$attr:meta])+ $id:ident : $t:ty = ($val:expr, $display:expr), $seealso:expr) => {
        $(#[$attr])*
        #[doc = $seealso]
        pub const $id: $t = $val;
    };
    ($(#[$lo_attr:meta])+ $lo:ident,
     $(#[$hi_attr:meta])+ $hi:ident : $t:ty = ($lo_val:expr, $hi_val:expr, $display:expr), $seealso:expr) => {
        $(#[$lo_attr])*
        #[doc = $seealso]
        pub const $lo: $t = $lo_val;
        $(#[$hi_attr])*
        #[doc = $seealso]
        pub const $hi: $t = $hi_val;
    };
}

macro_rules! declare_constants {
    ($t:ty, { $( $( $(#[$attr:meta])* $id:ident ),+ = ($($val:expr),+) ),+ }, $seealso:expr) => {
        $(
            declare_constant!($( $(#[$attr])* $id ),*: $t = ($($val),*), $seealso);
        )*
    }
}

macro_rules! match_constant_lhs {
    ($id:ident) => { $id };
    ($lo:ident, $hi:ident) => { $lo ... $hi };
}

macro_rules! match_constant_rhs {
    ($val:expr, $display:expr) => { $display };
    ($lo_val:expr, $hi_val:expr, $display:expr) => { $display };
}

macro_rules! add_doc {
    ($doc:expr, $item:item) => { #[doc = $doc] $item }
}

macro_rules! constants {
    ($name:ident, $t:ty, { $( $( $(#[$attr:meta])* $id:ident ),+ = ($($val:expr),+) ),+ }) => {
        declare_constants! {
            $t, { $( $( $(#[$attr])* $id ),+ = ($($val),+) ),+ },
            concat!("# See also\n",
                    "- [", stringify!($name), "](fn.", stringify!($name), ".html) -- Convert constant to `&str`.\n",
                    $($( "- [", stringify!($id), "](constant.", stringify!($id), ".html)\n"),*),*)
        }
        add_doc! {
            concat!("Returns a human-readable string representation of the corresponding ELF constants.\n\n",
                    "**Supported constants:**\n",
                    $($( "- [", stringify!($id), "](constant.", stringify!($id), ".html)\n"),*),*),
            pub fn $name(x: $t) -> &'static str {
                match x {
                    $( match_constant_lhs!( $($id),* ) => match_constant_rhs!( $($val),* ) ),*,
                    _ => "<unknown>",
                }
            }
        }
    }
}

macro_rules! relocations {
    ($name:ident, { $( $(#[$attr:meta])* $id:ident = $val:expr ),+ }) => {
        constants! { $name, Elf_Word, { $( $(#[$attr])* $id = ($val, stringify!($id)) ),* } }
    }
}

/*
macro_rules! constant_group {
    ($t:ty, { $( $( $(#[$attr:meta])* $id:ident ),+ = ($($val:expr),+) ),+ }) => {
        declare_constants! {
            $t, { $( $( $(#[$attr])* $id ),+ = ($($val),+) ),+ },
            concat!("# See also\n",
                    "- [", stringify!($name), "](fn.", stringify!($name), ".html)\n",
                    $($( "- [", stringify!($id), "](constant.", stringify!($id), ".html)\n"),*),*)
        }
    }
}
*/

