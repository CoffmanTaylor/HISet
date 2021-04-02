pub mod bag;
pub mod set;

#[macro_export]
macro_rules! hi_set {
    ($( $item:expr ),*) => {{
        let mut out = HISet::new();
        $(
            out.insert($item);
        )*
        out
    }};
}

#[macro_export]
macro_rules! hi_bag {
    ($( $item:expr ),*) => {{
        let mut out = HIBag::new();
        $(
            out.insert($item);
        )*
        out
    }};
}
