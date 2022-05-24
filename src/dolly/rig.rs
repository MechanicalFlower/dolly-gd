use gdnative::prelude::*;

///
pub struct RigUpdateParams<'a> {
    ///
    pub parent: &'a Transform,
    ///
    pub delta_time_seconds: f32,
}
