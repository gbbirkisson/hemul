use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::Tickable;

pub struct Oscillator {
    last_pass: Instant,
    delta: Duration,
    devices: Vec<(String, Box<dyn Tickable>)>,
}

impl Oscillator {
    fn new(delta: Duration) -> Self {
        Self {
            last_pass: Instant::now(),
            delta,
            devices: Vec::new(),
        }
    }

    pub fn from_hertz(hz: u64) -> Self {
        Self::new(Duration::from_nanos(1_000_000_000 / hz))
    }

    pub fn from_megahertz(mhz: f64) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Self::from_hertz((mhz * 1_000_000.0) as u64)
    }

    pub fn connect(&mut self, name: impl Into<String>, device: Box<dyn Tickable>) {
        self.devices.push((name.into(), device));
    }
}

impl Tickable for Oscillator {
    fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        let delta = now - self.last_pass;
        if delta > self.delta {
            for (name, device) in &mut self.devices {
                device
                    .tick()
                    .map_err(|e| format!("failed to tick '{name}': {e}"))?;
            }
            self.last_pass = now;
        }
        Ok(())
    }
}
