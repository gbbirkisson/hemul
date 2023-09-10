use crate::{Addressable, InteruptError, Interuptable};

use super::{Cpu, IRQB, NMIB};

impl<T> Interuptable for Cpu<T>
where
    T: Addressable,
{
    fn interupt(&mut self, tp: impl Into<crate::Interupt>) -> Result<(), InteruptError> {
        match tp.into() {
            0 => {
                if !self.I {
                    self.interupts.push_back(IRQB);
                }
            }
            _ => {
                // If non maskable interupt is not the first in queue, push it up front
                if self.interupts.get(0) != Some(&NMIB) {
                    self.interupts.push_front(NMIB);
                }
            }
        }
        Ok(())
    }
}
