use crate::dolly::{
    driver::RigDriver,
    rig::RigUpdateParams,
    utils::{ExpSmoothed, ExpSmoothingParams},
};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Smooths the parent transformation.
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct Smooth {
    /// Exponential smoothing factor for the position
    #[property(default = 1.0)]
    pub origin_smoothness: f32,

    /// Exponential smoothing factor for the rotation
    #[property(default = 1.0)]
    pub basis_smoothness: f32,

    /// Reverse the smoothing, causing the camera to look ahead of the parent transform
    ///
    /// This can be useful on top of [`Position`], and before another `Smooth`
    /// in the chain to create a soft yet responsive follower camera.
    ///
    /// [`Position`]: struct.Position.html
    /// [`Smooth`]: struct.Smooth.html
    #[property(default = false)]
    pub predictive: bool,

    smoothed_origin: ExpSmoothed<Vector3>,
    smoothed_basis: ExpSmoothed<Basis>,
}

#[methods]
impl Smooth {
    fn new(_owner: TRef<Resource>) -> Self {
        Self {
            origin_smoothness: 1.0,
            basis_smoothness: 1.0,
            predictive: false,
            smoothed_origin: Default::default(),
            smoothed_basis: Default::default(),
        }
    }
}

impl RigDriver for Smooth {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let origin = self.smoothed_origin.exp_smooth_towards(
            &params.parent.origin,
            ExpSmoothingParams {
                smoothness: self.origin_smoothness,
                predictive: self.predictive,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        let basis = self.smoothed_basis.exp_smooth_towards(
            &params.parent.basis,
            ExpSmoothingParams {
                smoothness: self.basis_smoothness,
                predictive: self.predictive,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        Transform { origin, basis }
    }
}
