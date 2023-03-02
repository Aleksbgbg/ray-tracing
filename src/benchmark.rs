extern crate test;

use crate::main;
use test::Bencher;

#[bench]
fn bench_render(bencher: &mut Bencher) {
  bencher.iter(|| main());
}
