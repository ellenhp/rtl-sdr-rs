use std::time::Duration;

use crate::error::Result;
use crate::error::RtlsdrError::RtlsdrErr;
use rusb::{Context, UsbContext};

use super::KNOWN_DEVICES;

#[derive(Debug)]
pub struct DeviceHandle {
    handle: rusb::DeviceHandle<Context>,
}
impl DeviceHandle {
    pub fn open(index: usize) -> Result<Self> {
        let mut context = Context::new()?;
        let handle = DeviceHandle::open_device(&mut context, index)?;
        Ok(DeviceHandle { handle })
    }

    pub fn open_device<T: UsbContext>(
        context: &mut T,
        mut index: usize,
    ) -> Result<rusb::DeviceHandle<T>> {
        let devices = context.devices()?;

        for found in devices.iter() {
            let device_desc = found.device_descriptor()?;
            for dev in KNOWN_DEVICES.iter() {
                if device_desc.vendor_id() == dev.vid && device_desc.product_id() == dev.pid {
                    if index == 0 {
                        let mut dev = found.open()?;
                        dev.set_auto_detach_kernel_driver(true).ok();
                        return Ok(dev);
                    } else {
                        index -= 1;
                    }
                }
            }
        }
        Err(RtlsdrErr("No device found".to_string()))
    }

    pub fn claim_interface(&mut self, iface: u8) -> Result<()> {
        Ok(self.handle.claim_interface(iface)?)
    }
    pub fn reset(&mut self) -> Result<()> {
        Ok(self.handle.reset()?)
    }

    pub fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<usize> {
        Ok(self
            .handle
            .read_control(request_type, request, value, index, buf, timeout)?)
    }

    pub fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration,
    ) -> Result<usize> {
        Ok(self
            .handle
            .write_control(request_type, request, value, index, buf, timeout)?)
    }

    pub fn read_bulk(&self, endpoint: u8, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.handle.read_bulk(endpoint, buf, timeout)?)
    }
}
