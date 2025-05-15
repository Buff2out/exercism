#[derive(Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    /*
    40 => 0 (0*60 + 40 - 60*0)
    60 => 1 (1*60 + 60 - 60*1)
    90 => 1 (1*60 + 90 - 60*1)
    150 => 150/60 = 2, (2*60 + 150 - 60*2)

    23 => 23 (23 - 24 * 0)
    26 => 2 (hours - 24 * hours_diff)
    49 => 1 (hours - 24 * hours_diff)
    */

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours_diff: i32 = (minutes as i32) / 60_i32;
        let hours = self.hours + hours_diff;
        let minutes = hours_diff*60 + minutes - 60*hours_diff;
        let hours_diff = hours / 24_i32;
        let hours = hours - 24 * hours_diff;
        Clock { hours, minutes }
    }
    pub fn to_string(&self) -> String {
        let hours = match self.hours.cmp(&10) {
            std::cmp::Ordering::Less => format!("0{}",self.hours),
            std::cmp::Ordering::Equal => format!("{}", self.hours),
            std::cmp::Ordering::Greater => format!("{}", self.hours),
        };
        let minutes = match self.minutes.cmp(&10) {
            std::cmp::Ordering::Less => format!("0{}",self.minutes),
            std::cmp::Ordering::Equal => format!("{}", self.minutes),
            std::cmp::Ordering::Greater => format!("{}", self.minutes),
        };
        format!("{}:{}", hours, minutes)
    }

}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        match self.hours.cmp(&other.hours) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => {
                match self.minutes.cmp(&other.minutes) {
                    std::cmp::Ordering::Less => false,
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => false,
                }
            },
            std::cmp::Ordering::Greater => false,
        }
    }
}
