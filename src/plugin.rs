use std::ffi::CString;
use std::sync::Once;

static mut PLUGIN: Option<Plugin> = None;

const PATCH_ADDRESS: usize = 0x74503E + 0x1;

pub struct Plugin {
    gtasa_userfiles_path: CString,
}

impl Drop for Plugin {
    fn drop(&mut self) {
        crate::utils::patch_pointer(PATCH_ADDRESS, 0x8747A8);
    }
}

impl Plugin {
    pub fn new() -> Plugin {
        let mut path = String::from("\\GTASA_");
        path.push_str(&std::env::current_dir().unwrap().to_str().unwrap()[3..].replace("\\", "_"));

        Plugin {
            gtasa_userfiles_path: CString::new(path).unwrap(),
        }
    }

    fn initialize_patchs(&self) {
        crate::utils::patch_pointer(PATCH_ADDRESS, self.gtasa_userfiles_path.as_ptr() as usize);
    }
}

pub fn initialize() {
    let plugin = Plugin::new();
    plugin.initialize_patchs();

    unsafe {
        PLUGIN = Some(plugin);
    }
}

pub fn uninitialize() {
    static DESTROY: Once = Once::new();

    DESTROY.call_once(|| unsafe {
        PLUGIN.take();
    });
}
