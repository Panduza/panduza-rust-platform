use std::ops::Deref;
use std::rc::Rc;

use tokio::signal;
use tokio::sync::mpsc;

use tokio::time::{sleep, Duration};



use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt;


use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};


use std::error::Error;
use std::cell::RefCell;





mod device;


mod platform;


#[tokio::main]
async fn main() {

    
    
    // let mut dv = device::Factory::new();

    // dv.add_producer("ddd".to_string(), Box::new(device::CustommmProducer{}) );

    // // dv.get_producer("ddd".to_string()).create_device();

    // let ddddd = "ddd".to_string();
    // let bbbb = dv.create_device( &ddddd ).unwrap();
    

    // println!("{}", bbbb.get_name());


    // let subscriber = tracing_subscriber::fmt()
    // // Use a more compact, abbreviated log format
    // .compact()
    // // Display source code file paths
    // .with_file(true)
    // // Display source code line numbers
    // .with_line_number(true)
    // // Display the thread ID an event was recorded on
    // .with_thread_ids(true)
    // // Don't display the event's target (module path)
    // .with_target(false)
    // // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    // .with_span_events(FmtSpan::FULL)
    // // Build the subscriber
    // .finish();

    // // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    
    // // console_subscriber::init();










    let mut platform_runner = platform::Runner::new();

    platform_runner.work().await;

    // let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<u8>();



    // let mut set =  ;
    
    // let pointer = Rc::new(set);








}

