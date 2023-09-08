use crate::{Addressable, InteruptError, Interuptable};

use super::{Cpu, IRQB, NMIB};

impl<T> Interuptable for Cpu<T>
where
    T: Addressable,
{
    fn interupt(&mut self, tp: impl Into<crate::Interupt>) -> Result<(), InteruptError> {
        match tp.into() {
            0 => {
                self.interupt_addr = Some(IRQB);
            }
            _ => {
                self.interupt_addr = Some(NMIB);
            }
        }
        Ok(())
    }
}
