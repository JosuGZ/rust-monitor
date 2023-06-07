pub struct Battery {
  manager: battery::Manager,
  battery: battery::Battery
}

pub struct BatteryData {
  pub percent: i32,
  pub rate: f32,
  pub hours: i32,
  pub minutes: i32
}

impl Battery {

  pub fn init() -> Option<Battery> {
    // We are silently ignoring errors for now

    let manager = battery::Manager::new();
    if manager.is_err() { return None; }
    let manager = manager.unwrap();


    let mut batteries = manager.batteries().ok()?;
    let battery = batteries.next()?.ok()?;

    Some(Battery { manager, battery })
  }

  pub fn discharging(&self) -> bool {
    self.battery.state() == battery::State::Discharging
  }

  pub fn refresh(&mut self) {
    // TODO: hide the battery or show an error
    // I don't know how likely is this. Probably if the battery is removed...
    _ = self.manager.refresh(&mut self.battery);
  }

  /// This should only be called when discharging
  pub fn get_data(&self) -> BatteryData {
    let battery = &self.battery;
    let percent = (battery.state_of_charge().value * 100.0) as i32;
    let rate = battery.energy_rate().value;

    let current_capacity = battery.energy().value * 2.777_777_8e-4;
    let remaining_hours = current_capacity / rate;

    let hours = remaining_hours.floor() as i32;
    let minutes = ((remaining_hours - remaining_hours.floor()) * 60.0) as i32;

    BatteryData {
      percent,
      rate,
      hours,
      minutes
    }
  }

}
