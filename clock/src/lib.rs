use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Self(0, 0).add_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let old_minutes = self.0 * 60 + self.1;
        let mut new_minutes = (old_minutes + minutes) % 1440;
        new_minutes = if new_minutes < 0 {
            1440 + new_minutes
        } else {
            new_minutes
        };
        Self(new_minutes / 60, new_minutes % 60)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}
