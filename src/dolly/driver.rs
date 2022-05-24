use crate::dolly::rig::RigUpdateParams;

use gdnative::prelude::*;

pub trait RigDriver: std::any::Any + std::fmt::Debug {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams) -> Transform;
}
