
use tokio::task::JoinSet;



pub struct Runner
{
    tasks: JoinSet<()>
}


impl Runner {


    
    pub fn new() -> Runner {
        return Runner {
            tasks: JoinSet::new()
        }
    }

    // #[tracing::instrument(level = "info")]
    // async fn sleepy_task(i:u8) {
        
    //     for number in 0..=10 {
    //         sleep(Duration::from_millis(1000)).await;
    //         tracing::warn!("{i} have elapsed {number}");
    //     }
    // }

    // #[tracing::instrument]
    // async fn task_waiter( &mut self) {

    //     while let Some(result) = self.tasks.join_next().await {
    //         println!("End task ");
    //     }
    // }

    // #[tracing::instrument]
    // async fn test(&mut self) {
    //     println!("pok");
    // }
    // // async fn test(&self) {
    // //     println!("pok");
    // // }

    // #[tracing::instrument]
    pub async fn work(&mut self) {

        tracing::info!("Platform");


    }


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


    //     let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
    //     mqttoptions.set_keep_alive(Duration::from_secs(5));

    //     let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
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



    //     // loop {} ???
    //     tokio::select! {
    //         _ = signal::ctrl_c() => {
    //             println!("end by user ctl-c");
                
    //         },
    //         // _ = shutdown_recv.recv() => {},
    //         _ = self.task_waiter() => {
    //             println!("end of all");
            
    //         }
    //     }
    
    // }
}

