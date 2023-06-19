use std::ffi::{c_char, CStr};

use ash::vk;
use raw_window_handle::HasRawDisplayHandle;

use crate::{
    vulkan::{debug::VulkanDebug, devices::Devices, swapchain::Swapchain},
    window::Window,
};

mod commands;
mod debug;
mod devices;
mod swapchain;

const LAYER_NAMES: [*const c_char; 1] =
    [unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }.as_ptr()];

pub struct Vulkan {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub devices: Devices,
    pub swapchain: Swapchain,
    pub commands: commands::Commands,
    pub debug_messenger: VulkanDebug,
}

impl Vulkan {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { ash::Entry::load() }.expect("vulkan loaded");

        let mut extenention_names =
            ash_window::enumerate_required_extensions(window.window.raw_display_handle())
                .unwrap()
                .to_vec();
        extenention_names.push(ash::extensions::ext::DebugUtils::name().as_ptr());

        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_1)
            .application_name(CStr::from_bytes_with_nul(b"Example Application\0").unwrap())
            .application_version(0)
            .engine_name(CStr::from_bytes_with_nul(b"Nilium\0").unwrap())
            .engine_version(0);

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&LAYER_NAMES)
            .enabled_extension_names(&extenention_names);

        let instance =
            unsafe { entry.create_instance(&create_info, None) }.expect("vulkan instance creation");

        let devices = Devices::new(&entry, &instance, window);

        let swapchain = Swapchain::new(&instance, &devices, window);

        let commands = commands::Commands::new(&devices);

        Self {
            debug_messenger: VulkanDebug::new(&entry, &instance),
            instance,
            entry,
            devices,
            swapchain,
            commands,
        }
    }
}
