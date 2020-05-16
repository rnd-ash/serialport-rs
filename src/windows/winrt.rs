use std::time::Duration;
use std::io;

use winrt::import;

// Now use the `import` macro to import the desired winmd files and modules:
import!(
    dependencies
        os
    types
        windows::devices::*
        windows::storage::streams::DataWriter
);
//        windows::devices::serialcommunication::SerialDevice
//        windows::devices::enumeration::*
//        windows::storage::streams::*

use crate::{
    ClearBuffer, DataBits, Error, ErrorKind, FlowControl, Parity, Result, SerialPort,
    SerialPortBuilder, SerialPortInfo, StopBits,
};

/// A serial port implementation for Windows COM ports.
///
/// The port will be closed when the value is dropped. However, this struct
/// should not be instantiated directly by using `COMPort::open()`, instead use
/// the cross-platform `serialport::open()` or
/// `serialport::open_with_settings()`.
#[derive(Debug)]
pub struct COMPort {
    timeout: Duration,
    port_name: Option<String>,
}

unsafe impl Send for COMPort {}

impl COMPort {
    /// Opens a COM port as a serial device.
    ///
    /// `port` should be the name of a COM port, e.g., `COM1`.
    ///
    /// If the COM port handle needs to be opened with special flags, use
    /// `from_raw_handle` method to create the `COMPort`. Note that you should
    /// set the different settings before using the serial port using `set_all`.
    ///
    /// ## Errors
    ///
    /// * `NoDevice` if the device could not be opened. This could indicate that
    ///    the device is already in use.
    /// * `InvalidInput` if `port` is not a valid device name.
    /// * `Io` for any other I/O error while opening or initializing the device.
    pub fn open(builder: &SerialPortBuilder) -> Result<COMPort> {
        //use windows::devices::serial_communication::SerialDevice;
        use windows::storage::streams::DataWriter;

        let mut name = Vec::<u16>::with_capacity(4 + builder.path.len() + 1);
        name.extend(r"\\.\".encode_utf16());
        name.extend(builder.path.encode_utf16());
        name.push(0);

        let selector = SerialDevice::GetDeviceSelector(name);

        Err(super::error::last_os_error())
    }
}

impl Drop for COMPort {
    fn drop(&mut self) {}
}

impl io::Read for COMPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "Operation timed out",
        ))
    }
}

impl io::Write for COMPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "Operation timed out",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "Operation timed out",
        ))
    }
}

impl SerialPort for COMPort {
    fn name(&self) -> Option<String> {
        self.port_name.clone()
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        Ok(())
    }

    fn write_request_to_send(&mut self, level: bool) -> Result<()> {
        Ok(())
    }

    fn write_data_terminal_ready(&mut self, level: bool) -> Result<()> {
        Ok(())
    }

    fn read_clear_to_send(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn read_data_set_ready(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn read_ring_indicator(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn read_carrier_detect(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn baud_rate(&self) -> Result<u32> {
        Ok(0)
    }

    fn data_bits(&self) -> Result<DataBits> {
        Err(Error::new(
            ErrorKind::Unknown,
            "Invalid data bits setting encountered",
        ))
    }

    fn parity(&self) -> Result<Parity> {
        Err(Error::new(
            ErrorKind::Unknown,
            "Invalid parity bits setting encountered",
        ))
    }

    fn stop_bits(&self) -> Result<StopBits> {
        Err(Error::new(
            ErrorKind::Unknown,
            "Invalid stop bits setting encountered",
        ))
    }

    fn flow_control(&self) -> Result<FlowControl> {
        Err(Error::new(
            ErrorKind::Unknown,
            "Invalid flow control setting encountered",
        ))
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> Result<()> {
        Ok(())
    }

    fn set_data_bits(&mut self, data_bits: DataBits) -> Result<()> {
        Ok(())
    }

    fn set_parity(&mut self, parity: Parity) -> Result<()> {
        Ok(())
    }

    fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<()> {
        Ok(())
    }

    fn set_flow_control(&mut self, flow_control: FlowControl) -> Result<()> {
        Ok(())
    }

    fn bytes_to_read(&self) -> Result<u32> {
        Ok(0)
    }

    fn bytes_to_write(&self) -> Result<u32> {
        Ok(0)
    }

    fn clear(&self, buffer_to_clear: ClearBuffer) -> Result<()> {
        Ok(())
    }

    fn try_clone(&self) -> Result<Box<dyn SerialPort>> {
        Err(super::error::last_os_error())
    }
}

/// List available serial ports on the system.
pub fn available_ports() -> Result<Vec<SerialPortInfo>> {
    let ports = Vec::new();
    Ok(ports)
}
