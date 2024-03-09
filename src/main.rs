use std::ops::Deref;
use std::rc::Rc;

use tokio::signal;
use tokio::sync::mpsc;

use tokio::time::{sleep, Duration};

use tokio::task::JoinSet;


use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt;


use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};


use std::error::Error;
use std::cell::RefCell;


#[derive(Debug)]
struct Platform
{
    tasks_group: JoinSet<()>,
    tasks_group_: RefCell<JoinSet<()>>
}


impl Platform {

    fn new() -> Platform {
        return Platform {
            tasks_group: JoinSet::new(),
            tasks_group_: RefCell::<JoinSet<()>>::new(JoinSet::new())
        }
    }

    #[tracing::instrument(level = "info")]
    async fn sleepy_task(i:u8) {
        
        for number in 0..=10 {
            sleep(Duration::from_millis(1000)).await;
            tracing::warn!("{i} have elapsed {number}");
        }
    }

    #[tracing::instrument]
    async fn task_waiter( &mut self) {

        while let Some(result) = self.tasks_group.join_next().await {
            println!("End task ");
        }
    }

    #[tracing::instrument]
    async fn test(&mut self) {
        println!("pok");
    }
    // async fn test(&self) {
    //     println!("pok");
    // }

    #[tracing::instrument]
    pub async fn run(&mut self) {
        println!("Hello, world!");


        // Create a channel with a capacity of 10 messages
        // let (tx, rx): (Sender<(i32, usize)>, Receiver<(i32, usize)>) = channel();


        let ff = RefCell::<u32>::new(5);
        
        *ff.borrow_mut() = 9;
        println!("{}", *ff.borrow());


        let newww = ff.clone();

        // let fut = ;
        self.tasks_group.spawn(Platform::sleepy_task(1));
        self.tasks_group.spawn(Platform::sleepy_task(2));

        self.tasks_group_.borrow_mut().spawn(Platform::sleepy_task(3));

        // let tttttt = self.tasks_group_.clone();

        // let ppp = self.tasks_group.clone();


        let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();


        self.tasks_group.spawn(async move {


            // tttttt.borrow_mut().spawn(Platform::sleepy_task(3));

            for i in 0..10 {
                println!("{}", *newww.borrow());
                client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
                time::sleep(Duration::from_millis(100)).await;
            }
        });

        self.tasks_group.spawn(async move {
            while let Ok(notification) = eventloop.poll().await {
                println!("Received = {:?}", notification);
            }
        });



        // loop {} ???
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("end by user ctl-c");
                
            },
            // _ = shutdown_recv.recv() => {},
            _ = self.task_waiter() => {
                println!("end of all");
            
            }
        }
    
    }
}



mod device;




#[tokio::main]
async fn main() {

    
    
    let dv = device::Factory::new();

    // dv.get_producer(ref).create_device()



    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .with_span_events(FmtSpan::FULL)
    // Build the subscriber
    .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    
    // console_subscriber::init();










    let mut platform = Platform::new();

    platform.run().await;

    // let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<u8>();



    // let mut set =  ;
    
    // let pointer = Rc::new(set);








}

