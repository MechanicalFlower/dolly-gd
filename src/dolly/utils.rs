use gdnative::prelude::*;

pub(crate) trait Interpolate {
    fn interpolate(self, other: Self, t: f32) -> Self;
}

impl Interpolate for Vector3 {
    fn interpolate(self, other: Self, t: f32) -> Self {
        self.linear_interpolate(other, t)
    }
}

impl Interpolate for Basis {
    fn interpolate(self, other: Self, t: f32) -> Self {
        // Technically should be a `slerp` for framerate independence, but the latter
        // will rotate in the negative direction when interpolating a 180..360 degree rotation
        // to the 0..180 range. See the comment about `yaw_degrees` in `YawPitch` for more details.
        // Quat::lerp(self.normalize(), other.normalize(), t).normalize()
        Self::from_quat(self.to_quat().slerp(other.to_quat(), t))
    }
}

pub(crate) struct ExpSmoothingParams {
    pub smoothness: f32,
    pub predictive: bool,
    pub delta_time_seconds: f32,
}

#[derive(Default, Debug)]
pub(crate) struct ExpSmoothed<T: Interpolate + Copy + std::fmt::Debug>(Option<T>);

impl<T: Interpolate + Copy + std::fmt::Debug> ExpSmoothed<T> {
    pub(crate) fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        // An ad-hoc multiplier to make default smoothness parameters
        // produce good-looking results.
        const SMOOTHNESS_MULT: f32 = 8.0;

        // Calculate the exponential blending based on frame time
        let interp_t = 1.0
            - (-SMOOTHNESS_MULT * params.delta_time_seconds / params.smoothness.max(1e-5)).exp();

        let prev = self.0.unwrap_or(*other);
        let smooth = prev.interpolate(*other, interp_t);

        self.0 = Some(smooth);

        #[allow(clippy::float_cmp)]
        if params.predictive {
            Interpolate::interpolate(*other, smooth, -1.0)
        } else {
            smooth
        }
    }
}

impl<T: FromVariant + Interpolate + Copy + std::fmt::Debug> FromVariant for ExpSmoothed<T> {
    #[inline]
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Option::<T>::from_variant(variant).map(|res| ExpSmoothed::<T>(res))
    }
}

impl<T: ToVariant + Interpolate + Copy + std::fmt::Debug> ToVariant for ExpSmoothed<T> {
    #[inline]
    fn to_variant(&self) -> Variant {
        self.0.to_variant()
    }
}
