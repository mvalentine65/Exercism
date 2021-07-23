use std::fmt;


#[derive(Debug,PartialEq)]
pub struct Clock {
        hours: i32,
        minutes: i32,
    }


fn find_minutes(hours:i32, minutes: i32) -> i32 {
    let new_minutes:i32 = (hours * 60 + minutes) % 1440;
    if new_minutes > 0 { return new_minutes; }
    else { return 1440 + new_minutes; }
}


impl fmt::Display for Clock {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
       write!(f, "{:02}:{:02}",self.hours, self.minutes) 
    }
}

impl Clock {
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes:i32 = find_minutes(hours, minutes);
        Self {
            hours: (total_minutes / 60) % 24,
            minutes: total_minutes % 60,
        }
    }
        
    pub fn add_minutes(&self, minutes: i32) -> Self {
           let total_minutes:i32 = find_minutes(self.hours, self.minutes + minutes);
           Self {
               hours: (total_minutes / 60) % 24,
               minutes: total_minutes % 60,
           }
    }
}
