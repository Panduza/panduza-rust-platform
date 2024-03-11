use tokio::signal;
use tokio::task::JoinSet;
use crate::device;

// use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rumqttc::{MqttOptions, AsyncClient, QoS};

pub struct Runner
{
    tasks: JoinSet<()>,
    device_factory: device::Factory

    // clients  HashMap<String, Box<dyn Producer>>
    // devices  HashMap<String, Box<dyn Producer>>

}

impl Runner {

    /// Create a new instance of the Runner
    pub fn new() -> Runner {
        return Runner {
            tasks: JoinSet::new(),
            device_factory: device::Factory::new()
        }
    }

    /// Main platform run loop
    pub async fn work(&mut self) {

        tracing::info!("Platform");


        // let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
        // mqttoptions.set_keep_alive(Duration::from_secs(5));

        // let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);



        // Wait for either a signal or all tasks to complete
        tokio::select! {
            _ = signal::ctrl_c() => {
                tracing::warn!("end by user ctrl-c");
            },
            _ = self.end_of_all_tasks() => {
                tracing::warn!("end by all tasks completed");
            }
        }
    }

    /// Wait for all tasks to complete
    async fn end_of_all_tasks( &mut self) {
        while let Some(result) = self.tasks.join_next().await {
            println!("End task ");
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
    //             client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
    //             time::sleep(Duration::from_millis(100)).await;
    //         }
    //     });

    //     self.tasks.spawn(async move {
    //         while let Ok(notification) = eventloop.poll().await {
    //             println!("Received = {:?}", notification);
    //         }
    //     });


}

