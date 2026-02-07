use super::Animation;

pub struct SunnyAnimation {
    frames: Vec<Vec<String>>,
    #[allow(dead_code)]
    frame_delay: u64,
}

impl SunnyAnimation {
    pub fn new() -> Self {
        let frames = vec![
            Self::create_frame_1(),
            Self::create_frame_2(),
            Self::create_frame_3(),
            Self::create_frame_4(),
        ];

        Self {
            frames,
            frame_delay: 500,
        }
    }

    fn create_frame_1() -> Vec<String> {
        vec![
            "    \\  |  /".to_string(),
            "     .-\"-.".to_string(),
            "--- (  O  ) ---".to_string(),
            "     `-.-'".to_string(),
            "    /  |  \\".to_string(),
        ]
    }

    fn create_frame_2() -> Vec<String> {
        vec![
            "     \\ | /".to_string(),
            "   .-\"   \"-.".to_string(),
            "-- (   O   ) --".to_string(),
            "   `-.   .-'".to_string(),
            "     / | \\".to_string(),
        ]
    }

    fn create_frame_3() -> Vec<String> {
        vec![
            "    /  |  \\".to_string(),
            "     .-\"-.".to_string(),
            "--- (  O  ) ---".to_string(),
            "     `-.-'".to_string(),
            "    \\  |  /".to_string(),
        ]
    }

    fn create_frame_4() -> Vec<String> {
        vec![
            "     / | \\".to_string(),
            "   .-\"   \"-.".to_string(),
            "-- (   O   ) --".to_string(),
            "   `-.   .-'".to_string(),
            "     \\ | /".to_string(),
        ]
    }
}

impl Animation for SunnyAnimation {
    fn get_frame(&self, frame_number: usize) -> Vec<String> {
        self.frames[frame_number % self.frames.len()].clone()
    }

    fn frame_count(&self) -> usize {
        self.frames.len()
    }

    fn frame_delay_ms(&self) -> u64 {
        self.frame_delay
    }
}

impl Default for SunnyAnimation {
    fn default() -> Self {
        Self::new()
    }
}
