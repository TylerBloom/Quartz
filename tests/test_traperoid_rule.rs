
#[cfg(test)]
mod tests {
    use quartz::integrate::one_d::trapezoid_integrator::{Integrator, TrapezoidIntegrator};
    
    fn linear(x: f64) -> f64 {
        2_f64*x + 3_f64
    }
    
    #[test]
    fn test_linear() {
        assert_eq!( TrapezoidIntegrator::<f64>::integrate_func(linear, 2.0_f64, 5.0_f64, 20), (5*5 + 5*3 - 2*2 - 2*3) as f64 );

    }

}
