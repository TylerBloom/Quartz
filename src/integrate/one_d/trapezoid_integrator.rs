
pub use std::marker::PhantomData;

pub use super::integrator::{Scalar, Integrator};

pub struct TrapezoidIntegrator<S> {
    _s: PhantomData<S>,
}

impl<S> TrapezoidIntegrator<S>
where S: Scalar 
{
    pub fn new() -> Self {
        TrapezoidIntegrator { _s: PhantomData }
    }
}

impl<S> Integrator<S> for TrapezoidIntegrator<S>
where S: Scalar
{
    // This uses a slimmed-down version of the trapezoid rule.
    // If we avoid double counting, it suffices to only divide the first and last element by two
    fn integrate_func(f: fn(S) -> S, a: S, b: S, steps: u64) -> S {
        if steps == 0 {
            S::zero()
        } else {
            let mut digest: S = f(a) * S::from(0.5);
            let dx: S = (b-a)/S::from(steps as f64); // NOTE: N is the number of sub-intervals here
            for i in 1..(steps)
            {
                digest += f(a + dx*S::from(i as f64));
            }
            digest += f(b) * S::from(0.5);
            digest * dx
        }
    }
    
    fn integrate_func_time_series(f: fn(S) -> S, times: Vec<S>) -> S {
        if times.len() == 0 {
            S::zero()
        } else {
            let mut digest = S::zero();
            let length = times.len();
            let mut past_x: S = times[0];
            let mut past_val: S = f(past_x);
            let mut curr_val: S;
            for i in 1..length
            {
                curr_val = f(times[i]);
                digest += (curr_val + past_val)*(times[i] - past_x);
                past_val = curr_val;
                past_x = times[i];
            }
            digest * S::from(0.5)
        }
    }
    
    fn integrate_data(vals: Vec<S>, dx: S) -> S {
        if vals.len() == 0 {
            S::zero()
        } else {
            let mut digest: S = vals[0] * S::from(0.5);
            let length = vals.len() - 1;
            for i in 1..length
            {
                digest += vals[i];
            }
            digest += *vals.last().unwrap() * S::from(0.5);
            digest * dx
        }
    }
    
    fn integrate_data_time_series(times: Vec<S>, vals: Vec<S>) -> S {
        if times.len() == 0 || vals.len() == 0 {
            S::zero()
        } else {
            let mut digest = S::zero();
            // TODO: If the length of the vectors aren't equal, what should we do?
            let length = if times.len() < vals.len() { times.len() } else { vals.len() };
            let mut past_x: S = times[0];
            let mut past_val: S = vals[0];
            let mut curr_val: S;
            for i in 1..length
            {
                curr_val = vals[i];
                digest += (curr_val + past_val)*(times[i] - past_x);
                past_val = curr_val;
                past_x = times[i];
            }
            digest * S::from(0.5)
        }
    }
    
}
