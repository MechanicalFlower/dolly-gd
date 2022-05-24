use crate::dolly::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::api::Resource;
use gdnative::prelude::*;

/// Locks/constrains the position of the camera to one or more axes
#[derive(NativeClass, FromVariant, ToVariant, Debug)]
#[inherit(Resource)]
pub struct LockPosition {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

#[methods]
impl LockPosition {
    pub fn new(_owner: TRef<Resource>) -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }
}

impl RigDriver for LockPosition {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let mut delta_pos = params.parent.origin;
        delta_pos.x = self.x.unwrap_or(delta_pos.x);
        delta_pos.y = self.y.unwrap_or(delta_pos.y);
        delta_pos.z = self.z.unwrap_or(delta_pos.z);
        Transform {
            origin: delta_pos,
            basis: params.parent.basis,
        }
    }
}