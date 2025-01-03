
#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: ((hours * 60 + minutes) % 1440 + 1440) % 1440
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: ((self.minutes + minutes) % 1440 + 1440) % 1440
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
