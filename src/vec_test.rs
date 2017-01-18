#[allow(unused_imports)]
use vec::*;

macro_rules! n_arr {
	[$i:ident, $v:expr, $N:expr] => ({
		let $i = 0;
		let mut arr = [$v; $N];
		for $i in 1..$N {
			arr[$i] = $v;
		}
		arr
	})
}

macro_rules! vn_content_test {
	($V:ident, $N:expr) => (
		let mut v = $V { d: n_arr![i, i + 1, $N] };
		for i in 0..$N {
			assert_eq!(v.d[i], i + 1);
		}

		let z = $V { d: [0; $N] };
		for i in 0..$N {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..$N {
			v.d[i] = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	vn_content_test!(v2, 2);
	vn_content_test!(v3, 3);
	vn_content_test!(v4, 4);
}

macro_rules! vn_copy_test {
	($V:ident, $N:expr) => (
		let v = $V::<usize> { d: n_arr![i, i + 1, $N] };
		let cv = v;

		for i in 0..$N {
			assert_eq!(cv.d[i], i + 1);
		}
	)
}

#[test]
fn copy() {
	vn_copy_test!(v2, 2);
	vn_copy_test!(v3, 3);
	vn_copy_test!(v4, 4);
}

macro_rules! vn_from_test {
	($V:ident, $N:expr) => (
		let vf: $V<usize> = $V::from(n_arr![i, i + 1, $N]);
		for i in 0..$N {
			assert_eq!(vf.d[i], i + 1);
		}

		let vi: $V<usize> = n_arr![i, i + 2, $N].into();
		for i in 0..$N {
			assert_eq!(vi.d[i], i + 2);
		}
	)
}

#[test]
fn from() {
	vn_from_test!(v2, 2);
	vn_from_test!(v3, 3);
	vn_from_test!(v4, 4);
}

macro_rules! vn_add_test {
	($V:ident, $N:expr) => (
		let va: $V<usize> = n_arr![i, i + 1, $N].into();
		let vb: $V<usize> = n_arr![i, i + 2, $N].into();
		let vc = va + vb;
		for i in 0..$N {
			assert_eq!(vc.d[i], va.d[i] + vb.d[i]);
		}
	)
}

#[test]
fn add() {
	vn_add_test!(v2, 2);
	vn_add_test!(v3, 3);
	vn_add_test!(v4, 4);
}