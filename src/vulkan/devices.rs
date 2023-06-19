use ash::{
    extensions::khr::{Surface, Swapchain},
    vk,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub mod queue;

pub struct Devices {
    pub physical_device: vk::PhysicalDevice,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: Surface,
    pub device: ash::Device,
    pub queues: queue::Queues,
}

impl Devices {
    pub fn new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &crate::window::Window,
    ) -> Self {
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.window.raw_display_handle(),
                window.window.raw_window_handle(),
                None,
            )
            .expect("surface creation")
        };

        let surface_loader = Surface::new(entry, instance);

        let physical_devices =
            unsafe { instance.enumerate_physical_devices() }.expect("vulkan physical devices");

        let (physical_device, queue_family_index) = physical_devices
            .iter()
            .find_map(|pd| unsafe {
                instance
                    .get_physical_device_queue_family_properties(*pd)
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let supports_graphics_and_surface =
                            info.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS)
                                && surface_loader
                                    .get_physical_device_surface_support(*pd, index as u32, surface)
                                    .expect("get physical device surface support");
                        if supports_graphics_and_surface {
                            Some((*pd, index))
                        } else {
                            None
                        }
                    })
            })
            .expect("suitable device");

        let queue_family_index = queue_family_index as u32;
        let device_extention_names_raw = [Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities);

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extention_names_raw)
            .enabled_features(&features);

        let device: ash::Device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("device created")
        };

        let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        let mut queues = queue::Queues::default();
        queues.insert(
            "present".into(),
            queue::Queue::new(present_queue, queue_family_index),
        );

        Self {
            physical_device,
            surface,
            device,
            queues,
            surface_loader,
        }
    }
}
