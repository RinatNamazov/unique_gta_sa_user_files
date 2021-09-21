use std::ffi::CString;

const PATCH_ADDRESS: usize = 0x74503E + 0x1;

pub struct Plugin {
    gtasa_userfiles_path: CString,
}

impl Plugin {
    pub fn new() -> Plugin {
        let cwd = std::env::current_dir().unwrap();
        let cwd = cwd.to_str().unwrap();
        let path = String::from("\\GTASA_") + &cwd[3..].replace("\\", "_");

        let plugin = Plugin {
            gtasa_userfiles_path: CString::new(path).unwrap(),
        };
        plugin.initialize_patchs();
        plugin
    }

    fn initialize_patchs(&self) {
        crate::utils::patch_pointer(PATCH_ADDRESS, self.gtasa_userfiles_path.as_ptr() as usize);
    }
}

