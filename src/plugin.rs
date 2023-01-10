use gdnative::api::{EditorPlugin, Resource, Script, Texture};
use gdnative::prelude::*;

#[derive(gdnative::derive::NativeClass)]
#[inherit(EditorPlugin)]
pub struct DollyPlugin {
    resource_names: Vec<String>,
}

#[methods]
impl DollyPlugin {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        DollyPlugin {
            resource_names: vec![
                "Arm".to_string(),
                "LockPosition".to_string(),
                "LookAt".to_string(),
                "Position".to_string(),
                "Rotation".to_string(),
                "Smooth".to_string(),
                "YawPitch".to_string(),
            ],
        }
    }

    #[method]
    fn _enter_tree(&self, #[base] owner: TRef<EditorPlugin>) {
        // Initialization of the plugin goes here.
        // Add the new type with a name, a parent type, a script and an icon.
        let script = unsafe {
            load::<Script>("res://addons/dolly-gd/native/DollyCamera.gdns", "Script").unwrap()
        };
        let texture = unsafe {
            load::<Texture>("res://addons/dolly-gd/icons/Camera3D.svg", "Texture").unwrap()
        };
        owner.add_custom_type("DollyCamera", "Camera", script, texture);

        for resource_name in self.resource_names.iter() {
            let script = unsafe {
                load::<Script>(
                    format!("res://addons/dolly-gd/native/{}.gdns", resource_name).as_str(),
                    "Script",
                )
                .unwrap()
            };
            let texture = unsafe {
                load::<Texture>("res://addons/dolly-gd/icons/Object.svg", "Texture").unwrap()
            };
            owner.add_custom_type(resource_name, "Resource", script, texture);
        }
    }

    #[method]
    fn _exit_tree(&self, #[base] owner: TRef<EditorPlugin>) {
        // Clean-up of the plugin goes here.
        // Always remember to remove it from the engine when deactivated.
        owner.remove_custom_type("DollyCamera");

        for resource_name in self.resource_names.iter() {
            owner.remove_custom_type(resource_name);
        }
    }
}

unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
where
    T: GodotObject<Memory = RefCounted> + SubClass<Resource>,
{
    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
    let resource = resource.assume_safe().claim();
    resource.cast::<T>()
}
