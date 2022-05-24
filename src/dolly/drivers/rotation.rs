use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Directly sets the rotation of the camera
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct Rotation {
    #[property]
    pub rotation: Basis,
}

#[methods]
impl Rotation {
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self { rotation: Basis::IDENTITY }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: params.parent.origin,
            basis: self.rotation,
        }
    }
}
