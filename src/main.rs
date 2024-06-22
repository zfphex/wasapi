use wasapi::*;

fn main() {
    unsafe {
        CoInitializeEx(ConcurrencyModel::MultiThreaded).unwrap();

        let enumerator = IMMDeviceEnumerator::new().unwrap();
        let default = enumerator
            .GetDefaultAudioEndpoint(DataFlow::Render, Role::Console)
            .unwrap();

        let collection = enumerator
            .EnumAudioEndpoints(DataFlow::All, DeviceState::Disabled)
            .unwrap();

        let devices: Vec<IMMDevice> = (0..collection.GetCount().unwrap())
            .map(|i| collection.Item(i).unwrap())
            .collect();

        println!("Default: {}", default.name());

        for device in devices {
            println!("{}", device.name());
        }
    }
}
