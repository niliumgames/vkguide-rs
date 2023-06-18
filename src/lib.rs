mod vulkan;
mod window;

pub struct NiliumEngine {
    pub frame_number: usize,
    pub vulkan: vulkan::Vulkan,
    pub window: window::Window,
}

impl NiliumEngine {
    pub fn new(width: u32, height: u32) -> Self {
        let window = window::Window::new(width, height);

        let vulkan = vulkan::Vulkan::new(&window);

        Self {
            frame_number: 0,
            window,
            vulkan,
        }
    }

    pub fn draw(&self) {}

    pub fn run(&mut self) {
        while !self.window.window.should_close() {
            // Poll and process events
            self.window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.window.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window.window.set_should_close(true)
                    }
                    _ => {}
                }
            }
        }
    }
}
