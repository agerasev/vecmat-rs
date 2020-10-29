#![allow(unused_macros)]


macro_rules! cartesian_1 {
	($m:ident, $x:tt, [$( $y:tt ),*]) => (
		$(
			$m!($x, $y);
		)*

	);
}

macro_rules! cartesian {
	($m:ident, [$( $x:tt ),*], $y:tt) => (
		$(
			cartesian_1!($m, $x, $y);
		)*
	);
}
