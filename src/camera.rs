use gdnative::api::Camera;
use gdnative::export::user_data::MapMut;
use gdnative::prelude::*;

use crate::dolly::{driver::RigDriver, drivers::*, rig::RigUpdateParams};

/// A chain of drivers, calculating displacements, and animating in succession.
#[derive(NativeClass)]
#[inherit(Camera)]
pub struct DollyCamera {
    #[property]
    components: VariantArray,
}

#[methods]
impl DollyCamera {
    fn new(_owner: TRef<Camera>) -> Self {
        DollyCamera {
            components: VariantArray::new_shared(),
        }
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    #[method]
    fn _process(&mut self, #[base] owner: TRef<Camera>, delta: f32) {
        let mut parent_transform = Transform::IDENTITY;

        if let Some(behavior_node) = owner.get_node("Behavior").as_ref() {
            let behavior_node = unsafe { behavior_node.assume_safe() };

            // Custom update
            unsafe { behavior_node.call_deferred("_process_camera", &[Variant::new(delta)]) };

            for component in self.components.iter() {
                if update_driver::<Arm>(&component, &mut parent_transform, delta).is_some()
                    || update_driver::<LockPosition>(&component, &mut parent_transform, delta)
                        .is_some()
                    || update_driver::<LookAt>(&component, &mut parent_transform, delta).is_some()
                    || update_driver::<Position>(&component, &mut parent_transform, delta).is_some()
                    || update_driver::<Rotation>(&component, &mut parent_transform, delta).is_some()
                    || update_driver::<Smooth>(&component, &mut parent_transform, delta).is_some()
                    || update_driver::<YawPitch>(&component, &mut parent_transform, delta).is_some()
                {
                    continue;
                }
            }
        }

        owner.set_transform(parent_transform);
    }
}

///
fn update_driver<T>(component: &Variant, parent_transform: &mut Transform, delta: f32) -> Option<()>
where
    T: RigDriver + NativeClass,
    T::Base: GodotObject<Memory = RefCounted>,
    T::UserData: MapMut,
{
    component
        .to::<Instance<T, Shared>>()
        .as_ref()
        .map(|driver| {
            let driver = unsafe { driver.assume_safe() };
            driver
                .map_mut(|d, _o| {
                    // Apply driver update on the parent transform
                    let transform = d.update(RigUpdateParams {
                        parent: parent_transform,
                        delta_time_seconds: delta,
                    });
                    parent_transform.clone_from(&transform);
                })
                .ok()
        })
        .unwrap_or(None)
}
