use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Calculate camera rotation based on yaw and pitch angles.
///
/// The angles follow the [`right-hand rule`] for curve orientation, and assume
/// an `OpenGL`-style coordinate system, meaning that for a camera to rotate right,
/// a negative value of yaw should be provided, and for it to rotate up,
/// a positive value of pitch.
///
/// [`right-hand rule`]: https://en.wikipedia.org/wiki/Right-hand_rule#Curve_orientation_and_normal_vectors
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct YawPitch {
    /// [0..720)
    ///
    /// Note: Quaternions can encode 720 degrees of rotation, causing a slerp from 350 to 0 degrees
    /// to happen counter-intuitively in the negative direction; the positive direction would go through 720,
    /// thus being farther. By encoding rotation here in the 0..720 range, we reduce the risk of this happening.
    #[property(default = 0.0)]
    pub yaw_degrees: f32,

    /// [-90..90]
    #[property(default = 0.0)]
    pub pitch_degrees: f32,
}

#[methods]
impl YawPitch {
    /// Creates camera looking forward along Z axis (negative or positive depends on system handedness)
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self {
            yaw_degrees: 0.0,
            pitch_degrees: 0.0,
        }
    }
}

impl RigDriver for YawPitch {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: params.parent.origin,
            basis: Basis::from_euler(Vector3::new(
                self.yaw_degrees * 180.0 / std::f32::consts::PI,
                self.pitch_degrees * 180.0 / std::f32::consts::PI,
                0.0,
            )),
        }
    }
}
