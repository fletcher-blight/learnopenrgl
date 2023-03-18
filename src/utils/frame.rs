pub struct Frame {
    start: std::time::Instant,
    last: std::time::Instant,
}

pub struct FrameInstant {
    start: std::time::Instant,
    last: std::time::Instant,
    now: std::time::Instant,
}

impl Frame {
    pub fn new_now() -> Self {
        let now = std::time::Instant::now();
        Frame {
            start: now,
            last: now,
        }
    }

    pub fn mark_new_frame(&mut self) -> FrameInstant {
        let now = std::time::Instant::now();
        let last = self.last;
        self.last = now;

        FrameInstant {
            start: self.start,
            last,
            now,
        }
    }
}

impl FrameInstant {
    pub fn total_duration(&self) -> std::time::Duration {
        self.now - self.start
    }

    pub fn last_frame_duration(&self) -> std::time::Duration {
        self.now - self.last
    }
}
