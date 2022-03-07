// modify this so worker can communicate main thread
// will need to create two channels on this one
//
use std::thread;
use crossbeam_channel::unbounded;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg{
    SumResult(i64),
    WorkerQuit,
}

fn main () {
    // need to create channel first
    let (worker_s, worker_r) = unbounded();
    let (main_s, main_r) = unbounded();

    // inifinite loop thread that receives messages
    let handle = thread::spawn(move || loop {
        match worker_r.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker {}", d),
                // when there is a sum, send back to main thread
                WorkerMsg::Sum(x,y) =>  {
                    main_s.send(MainMsg::SumResult(x + y));
                    ()
                },
                WorkerMsg::Quit => {
                    println!("Worker thread terminating");
                    main_s.send(MainMsg::WorkerQuit);

                    break;
                    }
            }
            Err(e) => {
                    println!("Worker Disconnected");
                    main_s.try_send(MainMsg::WorkerQuit);
                    break;
                }
            }
            
        
    });
    // everything sends to worker thread
    worker_s.send(WorkerMsg::PrintData("Hi".to_string()));
    worker_s.send(WorkerMsg::Sum(3,4));
    worker_s.send(WorkerMsg::Quit);

    // need thread back to main
    while let Ok(msg) = main_r.recv(){
        match msg {
            MainMsg::SumResult(answer) => println!("Main sum: {}", answer),
            MainMsg::WorkerQuit => println!("Main: Worker quit")

        }
    }
    handle.join();
}