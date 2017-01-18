#[allow(unused_imports)]
use mat::*;

macro_rules! mn_arr {
	[$i:ident, $j:ident; $v:expr; $N:expr] => ({
		let ($i, $j) = (0, 0);
		let mut arr = [$v; $N*$N];
		for $j in 0..$N {
			for $i in 0..$N {
				arr[$i + $j*$N] = $v;
			}
		}
		arr
	})
}

macro_rules! mn_content_test {
	($M:ident, $N:expr) => (
		let mut m = $M { d: mn_arr![i, j; i + j; $N] };
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(m.d[i + $N*j], i + j);
			}
		}

		let z = $M { d: [0; $N*$N] };
		for i in 0..($N*$N) {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..($N*$N) {
			m.d[i] = i + 2;
		}
		for i in 0..($N*$N) {
			assert_eq!(m.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	mn_content_test!(mat2, 2);
	mn_content_test!(mat3, 3);
	mn_content_test!(mat4, 4);
}

macro_rules! mn_copy_test {
	($M:ident, $N:expr) => (
		let m = $M::<usize> { d: mn_arr![i, j; i + j; $N] };
		let cm = m;

		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(cm.d[i + $N*j], i + j);
			}
		}
	)
}

#[test]
fn copy() {
	mn_copy_test!(mat2, 2);
	mn_copy_test!(mat3, 3);
	mn_copy_test!(mat4, 4);
}

macro_rules! mn_index_test {
	($M:ident, $N:expr) => (
		let mut m = $M::<usize> { d: mn_arr![i, j; i + j; $N] };
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i + j);
			}
		}
		for j in 0..$N {
			for i in 0..$N {
				m[(i, j)] = i*j;
			}
		}
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn index() {
	mn_index_test!(mat2, 2);
	mn_index_test!(mat3, 3);
	mn_index_test!(mat4, 4);
}