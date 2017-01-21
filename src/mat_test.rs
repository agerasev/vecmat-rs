#[allow(unused_imports)]
use num::{Zero, One};
#[allow(unused_imports)]
use mat::*;
#[allow(unused_imports)]
use vec::*;

macro_rules! vnm_arr {
	[$i:ident, $j:ident; $v:expr; $N:expr, $M:expr] => ({
		let ($i, $j) = (0, 0);
		let mut arr = [$v; $N*$M];
		for $j in 0..$M {
			for $i in 0..$N {
				arr[$i + $j*$N] = $v;
			}
		}
		arr
	})
}

macro_rules! vnm_content_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V { d: vnm_arr![i, j; i + j; $N, $M] };
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m.d[i + $N*j], i + j);
			}
		}

		let z = $V { d: [0; $N*$M] };
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..($N*$M) {
			m.d[i] = i + 2;
		}
		for i in 0..($N*$M) {
			assert_eq!(m.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	vnm_content_test!(mat2x2, 2, 2);
	vnm_content_test!(mat2x3, 2, 3);
	vnm_content_test!(mat2x4, 2, 4);
	vnm_content_test!(mat3x2, 3, 2);
	vnm_content_test!(mat3x3, 3, 3);
	vnm_content_test!(mat3x4, 3, 4);
	vnm_content_test!(mat4x2, 4, 2);
	vnm_content_test!(mat4x3, 4, 3);
	vnm_content_test!(mat4x4, 4, 4);
}

macro_rules! vnm_eq_test {
	($V:ident, $N:expr, $M:expr) => (
		let va = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		let vb = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	vnm_eq_test!(mat2x2, 2, 2);
	vnm_eq_test!(mat2x3, 2, 3);
	vnm_eq_test!(mat2x4, 2, 4);
	vnm_eq_test!(mat3x2, 3, 2);
	vnm_eq_test!(mat3x3, 3, 3);
	vnm_eq_test!(mat3x4, 3, 4);
	vnm_eq_test!(mat4x2, 4, 2);
	vnm_eq_test!(mat4x3, 4, 3);
	vnm_eq_test!(mat4x4, 4, 4);
}

macro_rules! vnm_copy_test {
	($V:ident, $N:expr, $M:expr) => (
		let m = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		let cm = m;
		assert_eq!(m, cm);
	)
}

#[test]
fn copy() {
	vnm_copy_test!(mat2x2, 2, 2);
	vnm_copy_test!(mat2x3, 2, 3);
	vnm_copy_test!(mat2x4, 2, 4);
	vnm_copy_test!(mat3x2, 3, 2);
	vnm_copy_test!(mat3x3, 3, 3);
	vnm_copy_test!(mat3x4, 3, 4);
	vnm_copy_test!(mat4x2, 4, 2);
	vnm_copy_test!(mat4x3, 4, 3);
	vnm_copy_test!(mat4x4, 4, 4);
}

macro_rules! vnm_index_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i + j);
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				m[(i, j)] = i*j;
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn index() {
	vnm_index_test!(mat2x2, 2, 2);
	vnm_index_test!(mat2x3, 2, 3);
	vnm_index_test!(mat2x4, 2, 4);
	vnm_index_test!(mat3x2, 3, 2);
	vnm_index_test!(mat3x3, 3, 3);
	vnm_index_test!(mat3x4, 3, 4);
	vnm_index_test!(mat4x2, 4, 2);
	vnm_index_test!(mat4x3, 4, 3);
	vnm_index_test!(mat4x4, 4, 4);
}

macro_rules! vnm_zero_test {
	($V:ident, $N:expr, $M:expr) => (
		let z = $V::<i32>::zero();
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}
		assert!(z.is_zero());
		
		let nz = $V::<i32> { d: [1; ($N*$M)] };
		assert!(!nz.is_zero());
	)
}

#[test]
fn zero() {
	vnm_zero_test!(mat2x2, 2, 2);
	vnm_zero_test!(mat2x3, 2, 3);
	vnm_zero_test!(mat2x4, 2, 4);
	vnm_zero_test!(mat3x2, 3, 2);
	vnm_zero_test!(mat3x3, 3, 3);
	vnm_zero_test!(mat3x4, 3, 4);
	vnm_zero_test!(mat4x2, 4, 2);
	vnm_zero_test!(mat4x3, 4, 3);
	vnm_zero_test!(mat4x4, 4, 4);
}

macro_rules! vnm_new_test {
	($V:ident, $N:expr, $M:expr) => (
		$V::<i32>::new();
	)
}

#[test]
fn new() {
	vnm_new_test!(mat2x2, 2, 2);
	vnm_new_test!(mat2x3, 2, 3);
	vnm_new_test!(mat2x4, 2, 4);
	vnm_new_test!(mat3x2, 3, 2);
	vnm_new_test!(mat3x3, 3, 3);
	vnm_new_test!(mat3x4, 3, 4);
	vnm_new_test!(mat4x2, 4, 2);
	vnm_new_test!(mat4x3, 4, 3);
	vnm_new_test!(mat4x4, 4, 4);
}


macro_rules! vnm_from_test {
	($V:ident, $N:expr, $M:expr) => (
		let mf: $V<usize> = $V::from(vnm_arr![i, j; i + j; $N, $M]);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mf[(i, j)], i + j);
			}
		}

		let mi: $V<usize> = vnm_arr![i, j; i*j; $N, $M].into();
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mi[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn from() {
	vnm_from_test!(mat2x2, 2, 2);
	vnm_from_test!(mat2x3, 2, 3);
	vnm_from_test!(mat2x4, 2, 4);
	vnm_from_test!(mat3x2, 3, 2);
	vnm_from_test!(mat3x3, 3, 3);
	vnm_from_test!(mat3x4, 3, 4);
	vnm_from_test!(mat4x2, 4, 2);
	vnm_from_test!(mat4x3, 4, 3);
	vnm_from_test!(mat4x4, 4, 4);
}

macro_rules! vnm_neg_test {
	($V:ident, $N:expr, $M:expr) => (
		let m: $V<i32> = vnm_arr![i, j; (i + j + 1) as i32; $N, $M].into();
		let nm = -m;
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(-m[(i, j)], nm[(i, j)]);
			}
		}
	)
}

#[test]
fn neg() {
	vnm_neg_test!(mat2x2, 2, 2);
	vnm_neg_test!(mat2x3, 2, 3);
	vnm_neg_test!(mat2x4, 2, 4);
	vnm_neg_test!(mat3x2, 3, 2);
	vnm_neg_test!(mat3x3, 3, 3);
	vnm_neg_test!(mat3x4, 3, 4);
	vnm_neg_test!(mat4x2, 4, 2);
	vnm_neg_test!(mat4x3, 4, 3);
	vnm_neg_test!(mat4x4, 4, 4);
}

macro_rules! vnm_op_mat_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let va = vnm_map![i, j; (2*(i + j) + 2) as i32; $V, $N, $M];
		let vb = vnm_map![i, j; (i + j + 1) as i32; $V, $N, $M];
		let vc = $op!(va, vb);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(vc[(i, j)], $op!(va[(i, j)], vb[(i, j)]));
			}
		}
	)
}

#[test]
fn mat_add() {
	vnm_op_mat_test!(mat2x2, 2, 2, op_add);
	vnm_op_mat_test!(mat2x3, 2, 3, op_add);
	vnm_op_mat_test!(mat2x4, 2, 4, op_add);
	vnm_op_mat_test!(mat3x2, 3, 2, op_add);
	vnm_op_mat_test!(mat3x3, 3, 3, op_add);
	vnm_op_mat_test!(mat3x4, 3, 4, op_add);
	vnm_op_mat_test!(mat4x2, 4, 2, op_add);
	vnm_op_mat_test!(mat4x3, 4, 3, op_add);
	vnm_op_mat_test!(mat4x4, 4, 4, op_add);
}

#[test]
fn mat_sub() {
	vnm_op_mat_test!(mat2x2, 2, 2, op_sub);
	vnm_op_mat_test!(mat2x3, 2, 3, op_sub);
	vnm_op_mat_test!(mat2x4, 2, 4, op_sub);
	vnm_op_mat_test!(mat3x2, 3, 2, op_sub);
	vnm_op_mat_test!(mat3x3, 3, 3, op_sub);
	vnm_op_mat_test!(mat3x4, 3, 4, op_sub);
	vnm_op_mat_test!(mat4x2, 4, 2, op_sub);
	vnm_op_mat_test!(mat4x3, 4, 3, op_sub);
	vnm_op_mat_test!(mat4x4, 4, 4, op_sub);
}

macro_rules! vnm_op_scal_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let v = vnm_map![i, j; (2*(i + j) + 2) as i32; $V, $N, $M];
		let a = 3;
		let va = $op!(v, a);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(va[(i, j)], $op!(v[(i, j)], a));
			}
		}
	)
}

#[test]
fn scal_mul() {
	vnm_op_scal_test!(mat2x2, 2, 2, op_mul);
	vnm_op_scal_test!(mat2x3, 2, 3, op_mul);
	vnm_op_scal_test!(mat2x4, 2, 4, op_mul);
	vnm_op_scal_test!(mat3x2, 3, 2, op_mul);
	vnm_op_scal_test!(mat3x3, 3, 3, op_mul);
	vnm_op_scal_test!(mat3x4, 3, 4, op_mul);
	vnm_op_scal_test!(mat4x2, 4, 2, op_mul);
	vnm_op_scal_test!(mat4x3, 4, 3, op_mul);
	vnm_op_scal_test!(mat4x4, 4, 4, op_mul);
}

#[test]
fn scal_div() {
	vnm_op_scal_test!(mat2x2, 2, 2, op_div);
	vnm_op_scal_test!(mat2x3, 2, 3, op_div);
	vnm_op_scal_test!(mat2x4, 2, 4, op_div);
	vnm_op_scal_test!(mat3x2, 3, 2, op_div);
	vnm_op_scal_test!(mat3x3, 3, 3, op_div);
	vnm_op_scal_test!(mat3x4, 3, 4, op_div);
	vnm_op_scal_test!(mat4x2, 4, 2, op_div);
	vnm_op_scal_test!(mat4x3, 4, 3, op_div);
	vnm_op_scal_test!(mat4x4, 4, 4, op_div);
}

#[test]
fn scal_rem() {
	vnm_op_scal_test!(mat2x2, 2, 2, op_rem);
	vnm_op_scal_test!(mat2x3, 2, 3, op_rem);
	vnm_op_scal_test!(mat2x4, 2, 4, op_rem);
	vnm_op_scal_test!(mat3x2, 3, 2, op_rem);
	vnm_op_scal_test!(mat3x3, 3, 3, op_rem);
	vnm_op_scal_test!(mat3x4, 3, 4, op_rem);
	vnm_op_scal_test!(mat4x2, 4, 2, op_rem);
	vnm_op_scal_test!(mat4x3, 4, 3, op_rem);
	vnm_op_scal_test!(mat4x4, 4, 4, op_rem);
}


macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vnm_op_mat_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let va = vnm_map![i, j; (2*(i + 2*j) + 2) as i32; $V, $N, $M];
		let vb = vnm_map![i, j; (i + j + 1) as i32; $V, $N, $M];
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}

#[test]
fn mat_add_assign() {
	vnm_op_mat_assign_test!(mat2x2, 2, 2, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat2x3, 2, 3, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat2x4, 2, 4, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat3x2, 3, 2, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat3x3, 3, 3, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat3x4, 3, 4, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat4x2, 4, 2, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat4x3, 4, 3, op_add_assign, op_add);
	vnm_op_mat_assign_test!(mat4x4, 4, 4, op_add_assign, op_add);
}

#[test]
fn mat_sub_assign() {
	vnm_op_mat_assign_test!(mat2x2, 2, 2, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat2x3, 2, 3, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat2x4, 2, 4, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat3x2, 3, 2, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat3x3, 3, 3, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat3x4, 3, 4, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat4x2, 4, 2, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat4x3, 4, 3, op_sub_assign, op_sub);
	vnm_op_mat_assign_test!(mat4x4, 4, 4, op_sub_assign, op_sub);
}

macro_rules! vnm_op_scal_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let v = vnm_map![i, j; (2*(i + 2*j) + 2) as i32; $V, $N, $M];
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}

#[test]
fn scal_mul_assign() {
	vnm_op_scal_assign_test!(mat2x2, 2, 2, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat2x3, 2, 3, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat2x4, 2, 4, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat3x2, 3, 2, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat3x3, 3, 3, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat3x4, 3, 4, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat4x2, 4, 2, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat4x3, 4, 3, op_mul_assign, op_mul);
	vnm_op_scal_assign_test!(mat4x4, 4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	vnm_op_scal_assign_test!(mat2x2, 2, 2, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat2x3, 2, 3, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat2x4, 2, 4, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat3x2, 3, 2, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat3x3, 3, 3, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat3x4, 3, 4, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat4x2, 4, 2, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat4x3, 4, 3, op_div_assign, op_div);
	vnm_op_scal_assign_test!(mat4x4, 4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	vnm_op_scal_assign_test!(mat2x2, 2, 2, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat2x3, 2, 3, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat2x4, 2, 4, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat3x2, 3, 2, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat3x3, 3, 3, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat3x4, 3, 4, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat4x2, 4, 2, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat4x3, 4, 3, op_rem_assign, op_rem);
	vnm_op_scal_assign_test!(mat4x4, 4, 4, op_rem_assign, op_rem);
}

macro_rules! vnm_transpose_test {
	($Vnm:ident, $Vmn:ident, $N:expr, $M:expr) => (
		let vnm = vnm_map![i, j; 2*i + 3*j; $Vnm, $N, $M];
		let vmn = vnm_map![i, j; 3*i + 2*j; $Vmn, $M, $N];
		assert_eq!(vnm.transpose(), vmn);
	)
}

#[test]
fn transpose() {
	vnm_transpose_test!(mat2x2, mat2x2, 2, 2);
	vnm_transpose_test!(mat2x3, mat3x2, 2, 3);
	vnm_transpose_test!(mat2x4, mat4x2, 2, 4);
	vnm_transpose_test!(mat3x2, mat2x3, 3, 2);
	vnm_transpose_test!(mat3x3, mat3x3, 3, 3);
	vnm_transpose_test!(mat3x4, mat4x3, 3, 4);
	vnm_transpose_test!(mat4x2, mat2x4, 4, 2);
	vnm_transpose_test!(mat4x3, mat3x4, 4, 3);
	vnm_transpose_test!(mat4x4, mat4x4, 4, 4);
}

macro_rules! vnm_outer_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let vn = vn_map![i; i as i32; $Vn, $N];
		let vm = vn_map![i; 2*i as i32; $Vm, $M];
		let mat = vnm_map![i, j; (2*i*j) as i32; $Vnm, $N, $M];
		let res = vm.outer(vn);
		assert_eq!(res, mat);
	)
}

#[test]
fn outer() {
	vnm_outer_test!(mat2x2, vec2, vec2, 2, 2);
	vnm_outer_test!(mat2x3, vec2, vec3, 2, 3);
	vnm_outer_test!(mat2x4, vec2, vec4, 2, 4);
	vnm_outer_test!(mat3x2, vec3, vec2, 3, 2);
	vnm_outer_test!(mat3x3, vec3, vec3, 3, 3);
	vnm_outer_test!(mat3x4, vec3, vec4, 3, 4);
	vnm_outer_test!(mat4x2, vec4, vec2, 4, 2);
	vnm_outer_test!(mat4x3, vec4, vec3, 4, 3);
	vnm_outer_test!(mat4x4, vec4, vec4, 4, 4);
}

macro_rules! vnm_mul_vec_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m: $Vnm<i32> = [1; $N*$M].into();
		let v: $Vn<i32> = [1; $N].into();
		assert_eq!(m*v, [$N; $M].into());
	)
}

#[test]
fn mul_vec() {
	vnm_mul_vec_test!(mat2x2, vec2, vec2, 2, 2);
	vnm_mul_vec_test!(mat2x3, vec2, vec3, 2, 3);
	vnm_mul_vec_test!(mat2x4, vec2, vec4, 2, 4);
	vnm_mul_vec_test!(mat3x2, vec3, vec2, 3, 2);
	vnm_mul_vec_test!(mat3x3, vec3, vec3, 3, 3);
	vnm_mul_vec_test!(mat3x4, vec3, vec4, 3, 4);
	vnm_mul_vec_test!(mat4x2, vec4, vec2, 4, 2);
	vnm_mul_vec_test!(mat4x3, vec4, vec3, 4, 3);
	vnm_mul_vec_test!(mat4x4, vec4, vec4, 4, 4);
}

macro_rules! vnm_mul_vec_mat_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m: $Vnm<i32> = [1; $N*$M].into();
		let v: $Vm<i32> = [1; $M].into();
		assert_eq!(v*m, [$M; $N].into());
	)
}

#[test]
fn mul_mat_vec() {
	vnm_mul_vec_mat_test!(mat2x2, vec2, vec2, 2, 2);
	vnm_mul_vec_mat_test!(mat2x3, vec2, vec3, 2, 3);
	vnm_mul_vec_mat_test!(mat2x4, vec2, vec4, 2, 4);
	vnm_mul_vec_mat_test!(mat3x2, vec3, vec2, 3, 2);
	vnm_mul_vec_mat_test!(mat3x3, vec3, vec3, 3, 3);
	vnm_mul_vec_mat_test!(mat3x4, vec3, vec4, 3, 4);
	vnm_mul_vec_mat_test!(mat4x2, vec4, vec2, 4, 2);
	vnm_mul_vec_mat_test!(mat4x3, vec4, vec3, 4, 3);
	vnm_mul_vec_mat_test!(mat4x4, vec4, vec4, 4, 4);
}

macro_rules! vnm_mul_mat_test {
	($Vnm:ident, $Vln:ident, $Vlm:ident, $N:expr, $M:expr, $L:expr) => (
		let vnm: $Vnm<i32> = [1; $N*$M].into();
		let vln: $Vln<i32> = [1; $L*$N].into();
		assert_eq!(vnm*vln, [$N; $L*$M].into());
	)
}

#[test]
fn mul_mat() {
	vnm_mul_mat_test!(mat2x2, mat2x2, mat2x2, 2, 2, 2);
	vnm_mul_mat_test!(mat2x2, mat3x2, mat3x2, 2, 2, 3);
	vnm_mul_mat_test!(mat2x2, mat4x2, mat4x2, 2, 2, 4);
	vnm_mul_mat_test!(mat2x3, mat2x2, mat2x3, 2, 3, 2);
	vnm_mul_mat_test!(mat2x3, mat3x2, mat3x3, 2, 3, 3);
	vnm_mul_mat_test!(mat2x3, mat4x2, mat4x3, 2, 3, 4);
	vnm_mul_mat_test!(mat2x4, mat2x2, mat2x4, 2, 4, 2);
	vnm_mul_mat_test!(mat2x4, mat3x2, mat3x4, 2, 4, 3);
	vnm_mul_mat_test!(mat2x4, mat4x2, mat4x4, 2, 4, 4);
	vnm_mul_mat_test!(mat3x2, mat2x3, mat2x2, 3, 2, 2);
	vnm_mul_mat_test!(mat3x2, mat3x3, mat3x2, 3, 2, 3);
	vnm_mul_mat_test!(mat3x2, mat4x3, mat4x2, 3, 2, 4);
	vnm_mul_mat_test!(mat3x3, mat2x3, mat2x3, 3, 3, 2);
	vnm_mul_mat_test!(mat3x3, mat3x3, mat3x3, 3, 3, 3);
	vnm_mul_mat_test!(mat3x3, mat4x3, mat4x3, 3, 3, 4);
	vnm_mul_mat_test!(mat3x4, mat2x3, mat2x4, 3, 4, 2);
	vnm_mul_mat_test!(mat3x4, mat3x3, mat3x4, 3, 4, 3);
	vnm_mul_mat_test!(mat3x4, mat4x3, mat4x4, 3, 4, 4);
	vnm_mul_mat_test!(mat4x2, mat2x4, mat2x2, 4, 2, 2);
	vnm_mul_mat_test!(mat4x2, mat3x4, mat3x2, 4, 2, 3);
	vnm_mul_mat_test!(mat4x2, mat4x4, mat4x2, 4, 2, 4);
	vnm_mul_mat_test!(mat4x3, mat2x4, mat2x3, 4, 3, 2);
	vnm_mul_mat_test!(mat4x3, mat3x4, mat3x3, 4, 3, 3);
	vnm_mul_mat_test!(mat4x3, mat4x4, mat4x3, 4, 3, 4);
	vnm_mul_mat_test!(mat4x4, mat2x4, mat2x4, 4, 4, 2);
	vnm_mul_mat_test!(mat4x4, mat3x4, mat3x4, 4, 4, 3);
	vnm_mul_mat_test!(mat4x4, mat4x4, mat4x4, 4, 4, 4);
}

macro_rules! vnm_one_test {
	($V:ident, $N:expr) => (
		let o = $V::<i32>::one();
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(o[(i, j)], if i == j { 1 } else { 0 });
			}
		}
	)
}

#[test]
fn one() {
	vnm_one_test!(mat2x2, 2);
	vnm_one_test!(mat3x3, 3);
	vnm_one_test!(mat4x4, 4);
}