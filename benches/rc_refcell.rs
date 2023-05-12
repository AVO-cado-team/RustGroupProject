#[macro_use]
extern crate bencher;
use bencher::Bencher;
use rust_group_project::{dll_trait::DoubleLinkedList, rc_refcell::Dll};

fn bench_create(bench: &mut Bencher) {
    bench.iter(Dll::<u16>::default);
}

fn bench_push_back(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    bench.iter(|| {
        dll.push_back(0);
    });
}

fn bench_push_front(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    bench.iter(|| {
        dll.push_front(0);
    });
}

fn bench_pop_back(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.pop_back();
    });
}

fn bench_pop_front(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.pop_front();
    });
}

fn bench_remove(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.remove(500);
    });
}

fn bench_find(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.find(&500);
    });
}

fn bench_get(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.get(500);
    });
}

fn bench_len(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.len();
    });
}

fn bench_is_empty(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.is_empty();
    });
}

fn bench_clear(bench: &mut Bencher) {
    let mut dll = Dll::<u16>::default();
    for i in 0..1000 {
        dll.push_back(i);
    }

    bench.iter(|| {
        dll.clear();
    });
}

benchmark_group!(
    benches,
    bench_create,
    bench_push_back,
    bench_push_front,
    bench_pop_back,
    bench_pop_front,
    bench_remove,
    bench_find,
    bench_get,
    bench_len,
    bench_is_empty,
    bench_clear
);
benchmark_main!(benches);
