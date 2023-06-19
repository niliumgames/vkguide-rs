use ash::vk;

#[derive(Debug, Copy, Clone)]
pub struct Queue {
    family_index: u32,
    queue: vk::Queue,
}

impl Queue {
    pub fn new(queue: vk::Queue, family_index: u32) -> Self {
        Self {
            queue,
            family_index,
        }
    }

    pub fn family_index(&self) -> u32 {
        self.family_index
    }

    pub fn queue(&self) -> vk::Queue {
        self.queue
    }
}

#[derive(Default, Debug, Clone)]
pub struct Queues(std::collections::HashMap<String, Queue>);

impl Queues {
    pub fn get(&self, key: &str) -> Option<&Queue> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, queue: Queue) {
        self.0.insert(key, queue);
    }
}
