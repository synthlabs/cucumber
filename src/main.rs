use std::ptr;
use windows::{
    core::*,
    Win32::{
        Media::Audio::*,
        System::Com::{StructuredStorage::PROPVARIANT, *},
        UI::Shell::PropertiesSystem::PROPERTYKEY,
    },
};

fn main() -> Result<()> {
    unsafe {
        CoInitialize(ptr::null_mut())?;
    }
    let enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };

    let devices = unsafe { enumerator.EnumAudioEndpoints(eAll, DEVICE_STATE_ACTIVE)? };

    let count = unsafe { devices.GetCount()? };

    println!("audio devices:");
    for i in 0..count {
        println!("--------------");
        let device = unsafe { devices.Item(i)? };

        let store = unsafe { device.OpenPropertyStore(StructuredStorage::STGM_READ)? };

        let key: *const PROPERTYKEY =
            &windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
        let props = unsafe { store.GetValue(key)? };

        let raw_name = unsafe { props.Anonymous.Anonymous.Anonymous.pwszVal };
        if raw_name.0.is_null() {
            return Ok(());
        }

        let mut end = raw_name.0;

        unsafe {
            while *end != 0 {
                end = end.add(1);
            }
        }

        let name = unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(
                raw_name.0,
                end.offset_from(raw_name.0) as _,
            ))
        };

        let raw_id = unsafe { device.GetId()? };

        if raw_id.0.is_null() {
            return Ok(());
        }

        let mut end = raw_id.0;

        unsafe {
            while *end != 0 {
                end = end.add(1);
            }
        }

        let id = unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(
                raw_id.0,
                end.offset_from(raw_id.0) as _,
            ))
        };

        println!("id: {id}");
        println!("name: {name}");

        let mut client: Option<IAudioClient> = None;
        unsafe {
            device.Activate(
                &IAudioClient::IID,
                CLSCTX_INPROC_SERVER,
                &PROPVARIANT::default(),
                &mut client as *mut _ as *mut _,
            )?;
        }

        let client = client.expect("expected to get audio client");

        let format = unsafe { *client.GetMixFormat()? };

        println!("wFormatTag: {}", { format.wFormatTag });
        println!("nChannels: {}", { format.nChannels });
        println!("nSamplesPerSec: {}", { format.nSamplesPerSec });
        println!("nAvgBytesPerSec: {}", { format.nAvgBytesPerSec });
        println!("nBlockAlign: {}", { format.nBlockAlign });
        println!("wBitsPerSample: {}", { format.wBitsPerSample });
        println!("cbSize: {}", { format.cbSize });

        //getdevicebyid https://docs.microsoft.com/en-us/windows/win32/coreaudio/capturing-a-stream
        //activate it https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudioclient-initialize
        //get audio client https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudioclient-getservice
        //have audio client accept data https://github.com/obsproject/obs-studio/blob/3c7139965080e29014639bb653a766f6a13d0e40/libobs/audio-monitoring/win32/wasapi-output.c

        unsafe { CoTaskMemFree(raw_name.0 as _) };
        unsafe { CoTaskMemFree(raw_id.0 as _) };
    }

    println!("Total: {:?}", count);

    Ok(())
}
