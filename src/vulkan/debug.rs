use ash::vk;

pub struct VulkanDebug {
    pub messenger: vk::DebugUtilsMessengerEXT,
}

impl VulkanDebug {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        let info = ash::vk::DebugUtilsMessengerCreateInfoEXT {
            message_type: (ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION),
            message_severity: (ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE),
            pfn_user_callback: Some(vulkan_debug_callback),
            ..Default::default()
        };

        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);
        let messenger = unsafe { debug_utils_loader.create_debug_utils_messenger(&info, None) }
            .expect("create debug messenger");

        Self { messenger }
    }
}

pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n"
    );

    ash::vk::FALSE
}
