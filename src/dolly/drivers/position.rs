use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Directly sets the position of the camera
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct Position {
    #[property]
    pub position: Vector3,
}

#[methods]
impl Position {
    ///
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self { position: Vector3::ZERO }
    }
}

impl RigDriver for Position {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: self.position,
            basis: params.parent.basis,
        }
    }
}
