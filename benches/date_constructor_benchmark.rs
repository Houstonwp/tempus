use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tempus::date::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("date constructor", |b| {
        b.iter(|| FieldDate {
            year: black_box(2000),
            month: black_box(Month(1)),
            day: black_box(Day(1)),
        })
    });
}

fn from_field_date(c: &mut Criterion) {
    let y = 2000_u32;
    let m = Month(1);
    let d = Day(1);
    let date = FieldDate {
        year: y,
        month: m,
        day: d,
    };

    c.bench_with_input(BenchmarkId::new("field to serial", date), &date, |b, &s| {
        b.iter(|| SerialDate::from(s))
    });
}

fn from_serial_date(c: &mut Criterion) {
    let x = SerialDate { rd: 16943 };

    c.bench_with_input(BenchmarkId::new("serial to field", x), &x, |b, &x| {
        b.iter(|| FieldDate::from(x));
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    from_field_date,
    from_serial_date
);
criterion_main!(benches);
