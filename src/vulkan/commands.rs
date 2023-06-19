use ash::vk;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub main_command_buffer: vk::CommandBuffer,
}

impl Commands {
    pub fn new(devices: &crate::vulkan::devices::Devices) -> Self {
        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(devices.queues.get("present").unwrap().family_index())
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool = unsafe {
            devices
                .device
                .create_command_pool(&command_pool_info, None)
                .expect("command pool created")
        };

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(1)
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY);

        let command_buffers = unsafe {
            devices
                .device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("allocate command buffer(s)")
        };

        let main_command_buffer = command_buffers[0];

        Self {
            command_pool,
            main_command_buffer,
        }
    }
}
