






mod log;
mod device;
mod platform;


#[tokio::main]
async fn main() {


    // log::Init("fmt");
    
    
    // let mut dv = device::Factory::new();

    // dv.add_producer("ddd".to_string(), Box::new(device::CustommmProducer{}) );

    // // dv.get_producer("ddd".to_string()).create_device();

    // let ddddd = "ddd".to_string();
    // let bbbb = dv.create_device( &ddddd ).unwrap();
    

    // println!("{}", bbbb.get_name());










    let mut platform_runner = platform::Runner::new();

    platform_runner.work().await;

    // let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<u8>();



    // let mut set =  ;
    
    // let pointer = Rc::new(set);








}

