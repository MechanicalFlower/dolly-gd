use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct Arm {
    ///
    #[property]
    pub offset: Vector3,
}

#[methods]
impl Arm {
    ///
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self { offset: Vector3::ZERO }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: params.parent.origin + params.parent.basis * self.offset,
            basis: params.parent.basis,
        }
    }
}
