use crate::dolly::{
    driver::RigDriver,
    rig::RigUpdateParams,
    utils::{ExpSmoothed, ExpSmoothingParams},
};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, and made to look ahead of it.
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct LookAt {
    /// Exponential smoothing factor
    #[property(default = 0.0)]
    pub smoothness: f32,

    /// The world-space position to look at
    #[property]
    pub target: Vector3,

    /// Reverse target position smoothing, causing the camera to look ahead of it.
    /// This can then be chained with [`Smooth`], to create
    /// a camera that smoothly follows an object, but doesn't lag far behind it.
    ///
    /// [`Smooth`]: struct.Smooth.html
    #[property(default = false)]
    pub predictive: bool,

    smoothed_target: ExpSmoothed<Vector3>,
}

#[methods]
impl LookAt {
    ///
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self {
            smoothness: 0.0,
            predictive: false,
            target: Vector3::ZERO,
            smoothed_target: Default::default(),
        }
    }
}

impl RigDriver for LookAt {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let target = self.smoothed_target.exp_smooth_towards(
            &self.target,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                predictive: self.predictive,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        params.parent.looking_at(target, Vector3::UP)
    }
}
