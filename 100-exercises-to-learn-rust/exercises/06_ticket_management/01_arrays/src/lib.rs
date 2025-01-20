// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

use std::i32::MIN;

pub struct WeekTemperatures {
    // TODO
    temperatures:[i32; 7]
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
        WeekTemperatures { temperatures: [MIN, MIN, MIN, MIN, MIN, MIN, MIN] }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        match day{
            Weekday::Monday => {
                if self.temperatures[0] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[0])
                }
            }
            Weekday::Tuesday => {
                if self.temperatures[1] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[1])
                }
            }
            Weekday::Wednesday => {
                if self.temperatures[2] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[2])
                }
            }
            Weekday::Thursday => {
                if self.temperatures[3] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[3])
                }
            }
            Weekday::Friday => {
                if self.temperatures[4] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[4])
                }
            }
            Weekday::Saturday => {
                if self.temperatures[5] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[5])
                }
            }
            Weekday::Sunday => {
                if self.temperatures[6] == MIN{
                    None
                }
                else {
                    Some(self.temperatures[6])
                }
            }
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        match day{
            Weekday::Monday => {
                self.temperatures[0] = temperature;
            }
            Weekday::Tuesday => {
                self.temperatures[1] = temperature;
            }
            Weekday::Wednesday => {
                self.temperatures[2] = temperature;
            }
            Weekday::Thursday => {
                self.temperatures[3] = temperature;
            }
            Weekday::Friday => {
                self.temperatures[4] = temperature;
            }
            Weekday::Saturday => {
                self.temperatures[5] = temperature;
            }
            Weekday::Sunday => {
                self.temperatures[6] = temperature;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
