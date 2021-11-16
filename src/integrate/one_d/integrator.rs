
pub use crate::math::scalar::Scalar;

pub trait Integrator<S>
where S: Scalar {
    fn integrate_func(f: fn(S) -> S, a: S, b: S, steps: u64) -> S;
    fn integrate_func_time_series(f: fn(S) -> S, times: Vec<S>) -> S;
    fn integrate_data(vals: Vec<S>, dx: S) -> S;
    fn integrate_data_time_series(times: Vec<S>, vals: Vec<S>) -> S;
}

