use tokio::signal;
use tokio::task::JoinSet;
use crate::device::Factory as DeviceFactory;
use crate::connection::Manager as ConnectionManager;

// use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rumqttc::{MqttOptions, AsyncClient, QoS};

pub struct Runner
{
    task_pool: JoinSet<()>,
    device_factory: DeviceFactory,
    connection_manager: ConnectionManager
    
    // devices  HashMap<String, Box<dyn Producer>>

}

impl Runner {

    /// Create a new instance of the Runner
    pub fn new() -> Runner {
        return Runner {
            task_pool: JoinSet::new(),
            device_factory: DeviceFactory::new(),
            connection_manager: ConnectionManager::new()
        }
    }

    /// Main platform run loop
    pub async fn work(&mut self) {

        // Info log
        tracing::info!("Platform Starting...");



        self.connection_manager.add_connection(&mut self.task_pool,"default".to_string(), "localhost".to_string(), 1883);




        
        // create connections
        // then devices
        // then attach devices to connections
        


        
        // I need to store client into a hashmap then I need to share clients with others tasks
        

        // let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);


        // client.publish("hello/rumqtt", QoS::AtLeastOnce, false, "pok").await.unwrap();

        // self.tasks.spawn(async move {
            
        //     loop {
        //         while let Ok(notification) = eventloop.poll().await {
        //             println!("Received = {:?}", notification);
        //         }
        //         tracing::warn!("Broker disconnected, trying to reconnect");
        //     }

        // });



        // Info log
        tracing::info!("Platform Started");

        // Wait for either a signal or all tasks to complete
        tokio::select! {
            _ = signal::ctrl_c() => {
                tracing::warn!("End by user ctrl-c");
            },
            _ = self.end_of_all_tasks() => {
                tracing::warn!("End by all tasks completed");
            }
        }
    }

    /// Wait for all tasks to complete
    async fn end_of_all_tasks( &mut self) {
        while let Some(result) = self.task_pool.join_next().await {
            tracing::info!("End task with result {:?}", result);
        }
    }

    // #[tracing::instrument(level = "info")]
    // async fn sleepy_task(i:u8) {
        
    //     for number in 0..=10 {
    //         sleep(Duration::from_millis(1000)).await;
    //         tracing::warn!("{i} have elapsed {number}");
    //     }
    // }




    //     // Create a channel with a capacity of 10 messages
    //     // let (tx, rx): (Sender<(i32, usize)>, Receiver<(i32, usize)>) = channel();


    //     let ff = RefCell::<u32>::new(5);
        
    //     *ff.borrow_mut() = 9;
    //     println!("{}", *ff.borrow());


    //     let newww = ff.clone();

    //     // let fut = ;
    //     self.tasks.spawn(Platform::sleepy_task(1));
    //     self.tasks.spawn(Platform::sleepy_task(2));

    //     self.tasks_group_.borrow_mut().spawn(Platform::sleepy_task(3));

    //     // let tttttt = self.tasks_group_.clone();

    //     // let ppp = self.tasks.clone();



    //     client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();


    //     self.tasks.spawn(async move {


    //         // tttttt.borrow_mut().spawn(Platform::sleepy_task(3));

    //         for i in 0..10 {
    //             println!("{}", *newww.borrow());
    //             
    //             time::sleep(Duration::from_millis(100)).await;
    //         }
    //     });




}

