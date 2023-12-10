use crate::command::Command;
use crate::registers::{Config, Register, Status};

/// Trait that hides all the GPIO/SPI type parameters for use by the
/// operation modes
pub trait Device {
    /// Error from the SPI implementation
    type Error;

    /// Set CE pin high
    fn ce_enable(&mut self);
    /// Set CE pin low
    fn ce_disable(&mut self);
    /// Helper; the receiving during RX and sending during TX require `CE`
    /// to be low.
    fn with_ce_disabled<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.ce_disable();
        let r = f(self);
        self.ce_enable();
        r
    }

    /// Send a command via SPI
    fn send_command<C: Command>(&mut self, command: &C) -> Result<(Status, C::Response), Self::Error>;
    /// Send `W_REGISTER` command
    fn write_register<R: Register>(&mut self, register: R) -> Result<Status, Self::Error>;
    /// Send `R_REGISTER` command
    fn read_register<R: Register>(&mut self) -> Result<(Status, R), Self::Error>;

    /// Read, and modify a register, and write it back if it has been changed.
    fn update_register<Reg, F, R>(&mut self, f: F) -> Result<R, Self::Error>
    where
        Reg: Register + PartialEq + Clone,
        F: FnOnce(&mut Reg) -> R,
    {
        // Use `update_config()` for `registers::Config`
        assert!(Reg::addr() != 0x00);

        let (_, old_register) = self.read_register::<Reg>()?;
        let mut register = old_register.clone();
        let result = f(&mut register);

        if register != old_register {
            self.write_register(register)?;
        }
        Ok(result)
    }

    /// Modify the (cached) `CONFIG` register and write if it has changed.
    fn update_config<F, R>(&mut self, f: F) -> Result<R, Self::Error>
    where
        F: FnOnce(&mut Config) -> R;
}
