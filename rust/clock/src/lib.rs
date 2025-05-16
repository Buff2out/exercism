use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

/*
    -2, -40 => 21, 20 ()
    -47, -20 => { -20 => -48, 40  } (-47/24 => -1; -1*24 + 47  = 23; -20)
*/

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let mut total_minutes = total_minutes % 1440;

        total_minutes = match total_minutes.cmp(&0) {
            Less => total_minutes + 1440,
            Equal | Greater => total_minutes,
        };

        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

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
        Clock::new(self.hours, self.minutes + minutes)
    }
    pub fn to_string(&self) -> String {
        let hours = match self.hours.cmp(&10) {
            Less => format!("0{}",self.hours),
            Equal | Greater => format!("{}", self.hours),
        };
        let minutes = match self.minutes.cmp(&10) {
            Less => format!("0{}",self.minutes),
            Equal | Greater => format!("{}", self.minutes),
        };
        format!("{}:{}", hours, minutes)
    }

}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        match self.hours.cmp(&other.hours) {
            Less | Greater => false,
            Equal => {
                match self.minutes.cmp(&other.minutes) {
                    Less | Greater => false,
                    Equal => true,
                }
            },
        }
    }
}
