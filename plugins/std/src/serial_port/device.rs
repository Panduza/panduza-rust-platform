use panduza_core::device::traits::DeviceActions;
use panduza_core::device::Device;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


/// Serial Port Device
/// 
pub struct SerialPort;


impl DeviceActions for SerialPort {

    /// Create the interfaces
    fn interface_builders(&self, device: &Device) 
        -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
    {

        // println!("Ka3005::interface_builders");
        // println!("{}", device_settings);

        // let mut serial_conf = SerialConfig::new();
        // serial_conf.import_from_json_settings(device_settings);

        // const_settings = {
        //     "usb_vendor": '0416',
        //     "usb_model": '5011',
        //     "serial_baudrate": 9600
        // }

        // serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        // list.push(
        //     itf_bpc::build("channel", &serial_conf)
        // );
        return Ok(list);
    }
}



