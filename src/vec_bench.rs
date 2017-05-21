use test::Bencher;
use vec::*;

fn init_ab4() -> (Vec4<f64>, Vec4<f64>) {
	(Vec4::<f64>::new_array([0.0, 1.0, 2.0, 3.0]), Vec4::<f64>::new_array([-4.0,-3.0,-2.0,-1.0]))
}

#[bench]
fn add(b: &mut Bencher) {
	let (va, vb) = init_ab4();
	b.iter(|| for _ in 0..1000 { va + vb; });
}

#[bench]
fn mul(b: &mut Bencher) {
	let (va, vb) = init_ab4();
	b.iter(|| for _ in 0..1000 { va*vb; });
}

#[bench]
fn dot(b: &mut Bencher) {
	let (va, vb) = init_ab4();
	b.iter(|| for _ in 0..1000 { va.dot(vb); });
}
