use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //calculate minutes
        let offset_hours = if minutes < 0 {
            let mut delta = minutes / 60;
            if minutes.rem_euclid(60) != 0 {
                delta -= 1;
            }
            delta
        } else {
            minutes / 60
        };

        let minutes = minutes.rem_euclid(60);
        
        //calculate hours
        let hours = (hours + offset_hours).rem_euclid(24);

        //create and return clock
        Clock {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        Self::new(self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

