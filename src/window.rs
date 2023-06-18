pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("glfw init");

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = glfw
            .create_window(width, height, "Vulkan Engine", glfw::WindowMode::Windowed)
            .expect("window created");

        window.set_key_polling(true);

        if !glfw.vulkan_supported() {
            panic!("Vulkan is not supported on this platform");
        }

        Self {
            glfw,
            window,
            events,
        }
    }
}
