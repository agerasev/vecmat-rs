#![allow(unused_macros)]

macro_rules! cartesian_1 {
	($m:ident, $x:ident, [$( $y:ident ),*]) => (
		$(
			$m!($x, $y);
		)*
		
	);
}

macro_rules! cartesian {
	($m:ident, [$( $x:ident ),*], $y:tt) => (
		$(
			cartesian_1!($m, $x, $y);
		)*
	);
}
