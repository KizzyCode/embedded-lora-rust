//! RFM95 SPI connection

use crate::err;
use crate::error::IoError;
use crate::rfm95::registers::Register;
use core::fmt::{Debug, Formatter};
use embedded_hal::spi::SpiDevice;

/// A RFM95 SPI connection
pub struct Rfm95Connection<Device>
where
    Device: SpiDevice,
{
    /// The SPI device
    device: Device,
}
impl<Device> Rfm95Connection<Device>
where
    Device: SpiDevice,
{
    /// A register read operation
    const RO: u8 = 0b0000_0000;
    /// A register write operation
    const RW: u8 = 0b1000_0000;

    /// Creates a new RFM95 SPI connection
    pub const fn init(device: Device) -> Self {
        Self { device }
    }

    /// Reads a RFM95 register via SPI
    pub fn read<T>(&mut self, register: T) -> Result<u8, IoError>
    where
        T: Register,
    {
        // Read register and extract (partial) value
        let register_value = self.register(Self::RO, register.address(), 0x00)?;
        Ok((register_value & register.mask()) >> register.offset())
    }
    /// Updates a RFM95 register via SPI
    pub fn write<T>(&mut self, register: T, value: u8) -> Result<(), IoError>
    where
        T: Register,
    {
        // Write the register
        if register.mask() == u8::MAX {
            // Fast-path as we overwrite the entire register
            self.register(Self::RW, register.address(), value)?;
        } else {
            // Read-Modify-Write of the register value to apply a partial update
            let old_value = self.register(Self::RO, register.address(), 0x00)?;
            let value = (old_value & !register.mask()) | (value << register.offset());
            self.register(Self::RW, register.address(), value)?;
        }

        // Operation successful
        Ok(())
    }

    /// Performs RFM95-specific SPI register access
    fn register(&mut self, operation: u8, address: u8, payload: u8) -> Result<u8, IoError> {
        // Build command
        let address = address & 0b0111_1111;
        let mut command = [operation | address, payload];

        // Do transaction
        self.device
            .transfer_in_place(&mut command)
            .map_err(|_| err!(IoError, "Failed to do GPIO operation or SPI transaction"))?;

        // SPI debug callback
        #[cfg(feature = "debug")]
        unsafe {
            extern "Rust" {
                /// Debug callback
                fn embeddedrfm95_spidebug_AwiUzTRu(operation: u8, address: u8, input: u8, output: u8);
            }

            // Call debug callback
            embeddedrfm95_spidebug_AwiUzTRu(operation, address, payload, command[1]);
        }

        // Return the previous register value
        Ok(command[1])
    }
}
impl<Device> Debug for Rfm95Connection<Device>
where
    Device: SpiDevice,
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.debug_struct("Rfm95Connection").field("device", &"<SpiDevice>").finish()
    }
}
