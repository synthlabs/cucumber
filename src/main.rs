use std::ptr;
use windows::{
    core::*,
    Win32::{Media::Audio::*, System::Com::*},
};

fn main() -> Result<()> {
    unsafe {
        CoInitialize(ptr::null_mut())?;
    }
    let enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
    println!("CoCreateInstance: {:?}", enumerator);

    let devices = unsafe { enumerator.EnumAudioEndpoints(eAll, DEVICE_STATE_ACTIVE)? };

    println!("GetCount: {:?}", unsafe { devices.GetCount()? });

    Ok(())
}
