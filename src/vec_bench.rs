use test::Bencher;
use vec::*;

#[bench]
fn bench_add_two(b: &mut Bencher) {
	let va = Vec4::<f64>::new_array([0.0, 1.0, 2.0, 3.0]);
	let vb = Vec4::<f64>::new_array([-4.0,-3.0,-2.0,-1.0]);
	b.iter(|| for _ in 0..1000 { va + vb; });
}