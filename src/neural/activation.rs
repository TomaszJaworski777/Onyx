pub trait ActivationFunction {
    fn execute(value: f32) -> f32;
}

#[derive(Clone, Copy, Default)]
pub struct NoActivation;
impl ActivationFunction for NoActivation {
    #[inline]
    fn execute(value: f32) -> f32 {
        value
    }
}

#[derive(Clone, Copy, Default)]
pub struct ScReLUActivation;
impl ActivationFunction for ScReLUActivation {
    #[inline]
    fn execute(value: f32) -> f32 {
        value.clamp(0.0, 1.0).powi(2)
    }
}

#[derive(Clone, Copy, Default)]
pub struct ReLUActivation;
impl ActivationFunction for ReLUActivation {
    #[inline]
    fn execute(value: f32) -> f32 {
        value.max(0.0)
    }
}

#[derive(Clone, Copy, Default)]
pub struct SigmoidActivation;
impl ActivationFunction for SigmoidActivation {
    #[inline]
    fn execute(value: f32) -> f32 {
        1.0 / (1.0 + (-value).exp())
    }
}
