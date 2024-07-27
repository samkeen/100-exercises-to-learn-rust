pub struct WeekTemperatures {
    temperatures: [Option<i32>; 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            temperatures: [None; 7],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        self.temperatures[day as usize]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        self.temperatures[day as usize] = Some(temperature);
    }
}
