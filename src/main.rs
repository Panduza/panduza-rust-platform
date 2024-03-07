use std::ops::Deref;
use std::rc::Rc;

use tokio::signal;
use tokio::sync::mpsc;

use tokio::time::{sleep, Duration};

use tokio::task::JoinSet;






struct Platform
{
    tasks_group: JoinSet<u8>
}


impl Platform {

    fn new() -> Platform {
        return Platform {
            tasks_group: JoinSet::new()
        }
    }


    async fn sleepy_task(i:u8) -> u8 {
        loop {
            sleep(Duration::from_millis(1000)).await;
            println!("{i} have elapsed");
            sleep(Duration::from_millis(1000)).await;
            println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            // sleep(Duration::from_millis(1000)).await;
            // println!("{i} have elapsed");
            return i;
        }
    }

    
    async fn task_waiter( &mut self) {
        while let Some(result) = self.tasks_group.join_next().await {
            println!("Task result: {}", result.unwrap());
        }
    }


    pub async fn run(&mut self) {
        println!("Hello, world!");

        // let fut = ;
        self.tasks_group.spawn(Platform::sleepy_task(1));
        self.tasks_group.spawn(Platform::sleepy_task(2));
        self.tasks_group.spawn(Platform::sleepy_task(3));



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




#[tokio::main]
async fn main() {

    let mut platform = Platform::new();

    platform.run().await;

    // let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<u8>();



    // let mut set =  ;
    
    // let pointer = Rc::new(set);








}

