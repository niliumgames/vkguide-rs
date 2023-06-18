use ash::vk;

use crate::vulkan::devices::Devices;

pub struct Swapchain {
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: ash::extensions::khr::Swapchain,
}

impl Swapchain {
    pub fn new(
        instance: &ash::Instance,
        devices: &Devices,
        window: &crate::window::Window,
    ) -> Self {
        let surface_format = unsafe {
            devices
                .surface_loader
                .get_physical_device_surface_formats(devices.physical_device, devices.surface)
                .unwrap()
        }[0];

        let surface_capabilities = unsafe {
            devices
                .surface_loader
                .get_physical_device_surface_capabilities(devices.physical_device, devices.surface)
                .unwrap()
        };
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count
        };

        let (width, height) = window.window.get_framebuffer_size();

        let surface_resolution = match surface_capabilities.current_extent.width {
            std::u32::MAX => vk::Extent2D {
                height: height as u32,
                width: width as u32,
            },
            _ => surface_capabilities.current_extent,
        };

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = unsafe {
            devices
                .surface_loader
                .get_physical_device_surface_present_modes(devices.physical_device, devices.surface)
                .unwrap()
        };
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, &devices.device);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(devices.surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap()
        };

        Self {
            swapchain,
            swapchain_loader,
        }
    }
}
