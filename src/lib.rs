use gdnative::prelude::*;

mod plugin;
mod camera;
mod dolly;

fn init(handle: InitHandle) {
    handle.add_tool_class::<plugin::DollyPlugin>();
    handle.add_class::<camera::DollyCamera>();
    handle.add_class::<dolly::drivers::Arm>();
    handle.add_class::<dolly::drivers::LockPosition>();
    handle.add_class::<dolly::drivers::LookAt>();
    handle.add_class::<dolly::drivers::Position>();
    handle.add_class::<dolly::drivers::Rotation>();
    handle.add_class::<dolly::drivers::Smooth>();
    handle.add_class::<dolly::drivers::YawPitch>();
}

godot_init!(init);