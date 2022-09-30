use criterion::{criterion_group, criterion_main, Criterion};
use curve25519_benches::{edwards::*, ristretto::*, scalar::*};

pub fn edwards_curve_validation(c: &mut Criterion) {
    let test_data = PodEdwardsPoint([
        120, 140, 152, 233, 41, 227, 203, 27, 87, 115, 25, 251, 219, 5, 84, 148, 117, 38, 84, 60,
        87, 144, 161, 146, 42, 34, 91, 155, 158, 189, 121, 79,
    ]);

    c.bench_function("edwards_curve_validation", |b| {
        b.iter(|| validate_edwards(&test_data))
    });
}

pub fn edwards_add(c: &mut Criterion) {
    let point_a = PodEdwardsPoint([
        33, 124, 71, 170, 117, 69, 151, 247, 59, 12, 95, 125, 133, 166, 64, 5, 2, 27, 90, 27, 200,
        167, 59, 164, 52, 54, 52, 200, 29, 13, 34, 213,
    ]);
    let point_b = PodEdwardsPoint([
        70, 222, 137, 221, 253, 204, 71, 51, 78, 8, 124, 1, 67, 200, 102, 225, 122, 228, 111, 183,
        129, 14, 131, 210, 212, 95, 109, 246, 55, 10, 159, 91,
    ]);

    c.bench_function("edwards_add", |b| {
        b.iter(|| add_edwards(&point_a, &point_b))
    });
}

pub fn edwards_subtract(c: &mut Criterion) {
    let point_a = PodEdwardsPoint([
        33, 124, 71, 170, 117, 69, 151, 247, 59, 12, 95, 125, 133, 166, 64, 5, 2, 27, 90, 27, 200,
        167, 59, 164, 52, 54, 52, 200, 29, 13, 34, 213,
    ]);
    let point_b = PodEdwardsPoint([
        70, 222, 137, 221, 253, 204, 71, 51, 78, 8, 124, 1, 67, 200, 102, 225, 122, 228, 111, 183,
        129, 14, 131, 210, 212, 95, 109, 246, 55, 10, 159, 91,
    ]);

    c.bench_function("edwards_subtract", |b| {
        b.iter(|| subtract_edwards(&point_a, &point_b))
    });
}

pub fn edwards_multiply(c: &mut Criterion) {
    let scalar = PodScalar([
        72, 191, 131, 55, 85, 86, 54, 60, 116, 10, 39, 130, 180, 3, 90, 227, 47, 228, 252, 99, 151,
        71, 118, 29, 34, 102, 117, 114, 120, 50, 57, 8,
    ]);
    let point = PodEdwardsPoint([
        176, 121, 6, 191, 108, 161, 206, 141, 73, 14, 235, 97, 49, 68, 48, 112, 98, 215, 145, 208,
        44, 188, 70, 10, 180, 124, 230, 15, 98, 165, 104, 85,
    ]);

    c.bench_function("edwards_multiply", |b| {
        b.iter(|| multiply_edwards(&scalar, &point))
    });
}

pub fn ristretto_curve_validation(c: &mut Criterion) {
    let test_data = PodRistrettoPoint([
        120, 140, 152, 233, 41, 227, 203, 27, 87, 115, 25, 251, 219, 5, 84, 148, 117, 38, 84, 60,
        87, 144, 161, 146, 42, 34, 91, 155, 158, 189, 121, 79,
    ]);

    c.bench_function("ristretto_curve_validation", |b| {
        b.iter(|| validate_ristretto(&test_data))
    });
}

pub fn ristretto_add(c: &mut Criterion) {
    let point_a = PodRistrettoPoint([
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98, 34,
        183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ]);
    let point_b = PodRistrettoPoint([
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99, 231,
        162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ]);

    c.bench_function("ristretto_add", |b| {
        b.iter(|| add_ristretto(&point_a, &point_b))
    });
}

pub fn ristretto_subtract(c: &mut Criterion) {
    let point_a = PodRistrettoPoint([
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98, 34,
        183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ]);
    let point_b = PodRistrettoPoint([
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99, 231,
        162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ]);

    c.bench_function("ristretto_subtract", |b| {
        b.iter(|| subtract_ristretto(&point_a, &point_b))
    });
}

pub fn ristretto_multiply(c: &mut Criterion) {
    let scalar = PodScalar([
        254, 198, 23, 138, 67, 243, 184, 110, 236, 115, 236, 205, 205, 215, 79, 114, 45, 250, 78,
        137, 3, 107, 136, 237, 49, 126, 117, 223, 37, 191, 88, 6,
    ]);
    let point = PodRistrettoPoint([
        68, 80, 232, 181, 241, 77, 60, 81, 154, 51, 173, 35, 98, 234, 149, 37, 1, 39, 191, 201,
        193, 48, 88, 189, 97, 126, 63, 35, 144, 145, 203, 31,
    ]);

    c.bench_function("ristretto_multiply", |b| {
        b.iter(|| multiply_ristretto(&scalar, &point))
    });
}

criterion_group!(
    benches,
    edwards_curve_validation,
    edwards_add,
    edwards_subtract,
    edwards_multiply,
    ristretto_curve_validation,
    ristretto_add,
    ristretto_subtract,
    ristretto_multiply
);
criterion_main!(benches);
