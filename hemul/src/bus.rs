use std::{
    error::Error,
    ops::{Index, IndexMut},
};

use crate::{Addressable, Byte, Snapshottable, Word};

#[derive(Default)]
pub struct Bus {
    devices: Vec<(String, Word, Word, Box<dyn Addressable>)>,
}

impl Bus {
    pub fn connect(
        &mut self,
        name: impl Into<String>,
        start: Word,
        end: Word,
        device: Box<dyn Addressable>,
    ) {
        self.devices.push((name.into(), start, end, device));
    }
}

impl Addressable for Bus {
    fn inside_bounds(&self, addr: Word) -> bool {
        for (_, _, _, device) in &self.devices {
            if device.inside_bounds(addr) {
                return true;
            }
        }
        false
    }
}

impl Index<Word> for Bus {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        for (_, start, end, device) in &self.devices {
            if *start <= index && index <= *end {
                return device.index(index);
            }
        }
        panic!("Indexed to unknown device")
    }
}

impl IndexMut<Word> for Bus {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        for (_, start, end, device) in &mut self.devices {
            if *start <= index && index <= *end {
                return device.index_mut(index);
            }
        }
        panic!("Indexed to unknown device")
    }
}

impl Snapshottable for Bus {
    type Snapshot = Vec<Byte>;

    fn snapshot(&self) -> Result<Self::Snapshot, Box<dyn Error>> {
        let mut end = Word::MIN;
        for (_, _, e, _) in &self.devices {
            if &end < e {
                end = *e;
            }
        }
        let mut dump = vec![0; end as usize];
        for (_, start, end, device) in &self.devices {
            for i in *start..=*end {
                dump[i as usize] = device[i];
            }
        }
        Ok(dump)
    }
}
