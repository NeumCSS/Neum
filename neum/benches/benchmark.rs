use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::rc::Rc;

use neum::Neum;

fn parsing_files(c: &mut Criterion) {
    let mut data = String::new();
    for i in walkdir::WalkDir::new(std::path::Path::new("src/default")) {
        let i = i
            .as_ref()
            .unwrap_or_else(|_| panic!("Cant get a file, {i:?}"));
        if i.file_type().is_file() {
            let file = i.path().display().to_string();
            let content = std::fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
            data.push_str(&content);
        }
    }

    let data = Rc::new(data.as_str());

    c.bench_function("parse defaults", |b| {
        b.iter(|| {
            Neum::new(*data.clone(), None).unwrap();
        })
    });
}

fn convert(c: &mut Criterion) {
    let default = Neum::default();
    c.bench_function("convert", |b| {
        b.iter_batched(
            || default.clone(),
            |mut default| {
                for i in vec![
                    "m-0",
                    "ds-lg",
                    "bg-white",
                    "w-screen",
                    "maw-360",
                    "m-auto",
                    "h-32",
                    "h-24",
                    "ml-5",
                    "f-left",
                    "center-xy",
                    "ds-lg",
                    "d-flex",
                    "mt--4",
                    "bc-teal",
                    "ds-lg",
                    "w-128",
                    "h-6.5",
                    "r",
                    "rr-none",
                    "bc-teal",
                    "bg-teal",
                    "w-8",
                    "h-8",
                    "r",
                    "rl-none",
                ] {
                    default.convert(i);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn init(c: &mut Criterion) {
    c.bench_function("initilize default", |b| {
        b.iter(|| {
            Neum::default();
        })
    });
}

fn add(c: &mut Criterion) {
    let mut data = String::new();
    for i in walkdir::WalkDir::new(std::path::Path::new("src/default")) {
        let i = i
            .as_ref()
            .unwrap_or_else(|_| panic!("Cant get a file, {i:?}"));
        if i.file_type().is_file() {
            let file = i.path().display().to_string();
            let content = std::fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
            data.push_str(&content);
        }
    }

    let data = Rc::new(data.as_str());

    c.bench_function("add", |b| {
        b.iter(|| {
            Neum::empty().add(*data.clone(), None).unwrap();
        })
    });

    c.bench_function("add priority", |b| {
        b.iter(|| {
            Neum::empty().add_priority(*data.clone(), None).unwrap();
        })
    });
}

fn combine(c: &mut Criterion) {
    let mut data = String::new();
    for i in walkdir::WalkDir::new(std::path::Path::new("src/default")) {
        let i = i
            .as_ref()
            .unwrap_or_else(|_| panic!("Cant get a file, {i:?}"));
        if i.file_type().is_file() {
            let file = i.path().display().to_string();
            let content = std::fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
            data.push_str(&content);
        }
    }

    let data = Rc::new(Neum::new(data.as_str(), None).unwrap());

    c.bench_function("combine", |b| {
        b.iter(|| {
            Neum::empty().combine(Rc::make_mut(&mut data.clone()));
        })
    });

    c.bench_function("combine priority", |b| {
        b.iter(|| {
            Neum::empty().combine_priority(Rc::make_mut(&mut data.clone()));
        })
    });
}

criterion_group!(benches, parsing_files, convert, init, add, combine);
criterion_main!(benches);
