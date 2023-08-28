use std::time::{Duration, Instant};

use crate::{TickError, Tickable};

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
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Self::new(Duration::from_nanos(1_000_000_000 / hz))
    }

    pub fn from_megahertz(mhz: u64) -> Self {
        Self::from_hertz(mhz * 1_000_000)
    }

    pub fn connect(&mut self, name: impl Into<String>, device: Box<dyn Tickable>) {
        self.devices.push((name.into(), device));
    }
}

impl Tickable for Oscillator {
    fn tick(&mut self) -> Result<(), TickError> {
        let now = Instant::now();
        let delta = self.last_pass - now;
        if delta > self.delta {
            for (name, device) in &mut self.devices {
                device
                    .tick()
                    .map_err(|e| format!("Failed to tick '{name}': {e}"))?;
            }
            self.last_pass = now;
        }
        Ok(())
    }
}
