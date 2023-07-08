use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use text_io::read;
use std::sync::mpsc;


#[derive(Clone)]
#[derive(Debug)]
struct Request {
    id: u32,
    data: String,
}

#[derive(Clone)]
#[derive(PartialEq)]
struct Response {
    id: u32,
    result: String,
}

struct RequestQueue {
    queue: Vec<Request>,
    max_size: usize,
}

impl RequestQueue {
    fn new(max_size: usize) -> Self {
        RequestQueue {
            queue: Vec::new(),
            max_size,
        }
    }

    fn enqueue(&mut self, request: Request) -> bool {
        if self.queue.len() >= self.max_size {
            false
        } else {
            self.queue.push(request);
            true
        }
    }

    fn dequeue(&mut self) -> Option<Request> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}

#[derive(Clone)]
struct ResponseQueue {
    queue: Vec<(Response, Instant)>,
}

impl ResponseQueue {
    fn new() -> Self {
        ResponseQueue { queue: Vec::new() }
    }

    fn enqueue(&mut self, response: Response) {
        self.queue.push((response, Instant::now()));
    }

fn dequeue(&mut self, timeout: Duration) -> Option<Response> {
        let start = Instant::now();
        loop {
            
            if let Some(rresponse)= self.queue.pop() {
                return Some(rresponse.0);
            }
            if start.elapsed() >= timeout {
                return None;
            }
        }
    }
}




fn heavy_processing(request: Request, processing_period: Duration, max_wait_time: Duration) -> Option<Response> {
        thread::sleep(processing_period);

        let start_time = Instant::now();

            let r=Response {
                id: (request.id),
                result:(request.data+" in heavy processing"),
            };

            if start_time.elapsed() >= max_wait_time {
                print!("Timeout");
                return None;
            }
            return Some(r);

}

fn handle_request(request: Request, response_queue: Arc<Mutex<ResponseQueue>>, max_wait_time: Duration) -> Option<Response> {

    let mut responses = response_queue.lock().unwrap();
    let start_time = Instant::now();

    let responsee: Response= Response {
        id:request.id,
        result:request.data,};
        responses.enqueue(responsee);
    loop {
        if let Some(response) = responses.dequeue(max_wait_time.checked_sub(start_time.elapsed()).unwrap_or_default()) {
            return Some(response);
        }
        if start_time.elapsed() >= max_wait_time {
            print!("Timeout");
            return None;
        }
        thread::sleep(Duration::from_secs(5));
    }
}




fn main() {
    // Parameters from server configuration
    println!("Please enter heavy processing period (in seconds):");
    let heavy_processing_period_input: usize = read!();
    let heavy_processing_period = Duration::from_secs(heavy_processing_period_input as u64);

    println!("Please enter max queue size:");
    let max_queue_size: usize = read!();

    println!("Please enter max response wait time (in seconds):");
    let max_response_wait_time_input: usize = read!();
    let max_response_wait_time = Duration::from_secs(max_response_wait_time_input as u64);

    // let _rq1 = Request {
    //     id: 3,
    //     data: String::from("salam"),};

    // let _rq2 = Request {
    //     id: 4,
    //     data: String::from("sa"),};




    let mut handles= Vec::new();
    

for _j in 0..2 {
    let counter = Arc::new(Mutex::new(0));
let (tx, rx) = mpsc::channel();


let mut input_queue = RequestQueue::new(max_queue_size);
let mut resultq=ResponseQueue::new();
// let mut input_queue: std::sync::MutexGuard<'_, RequestQueue> = request_queue.lock().unwrap();

let mut handle;

for i in 0..5{
    let tx1 = tx.clone();
    let x=Arc::new(Request { id: (i), data: ( format!("Request #{}", i)) });
    let xc=Arc::new(Request { id: (i), data: ( format!("Request #{}", i)) });
    
    let response_queue = Arc::new(Arc::new(Mutex::new(ResponseQueue::new())));

    if input_queue.queue.len()<max_queue_size {

        input_queue.enqueue((*xc).clone());
        let counter = Arc::clone(&counter);
        handle=thread::spawn(move || {
              let r: Option<Response>= handle_request((*x).clone(), (*response_queue).clone(), heavy_processing_period);
              
              match r {
                  Some(p)=>              {
                    {
                        tx1.send(p).unwrap();
                        let mut num = counter.lock().unwrap();
                        *num += 1;

                    }
                  }
                  None=> {print!("request : timeout");}
              } 

              

            
            });
            handles.push(handle);

        

 
    }
    else {
            handle= thread::spawn(move || {
                let r=heavy_processing((*x).clone(),heavy_processing_period, max_response_wait_time);
                match r {
                    Some(p)=>              {
                      {
                          tx1.send(p).unwrap();
                      }
                    }
                    None=> {print!("request : timeout");}
                }
            });
            handles.push(handle);
        }



        while *counter.lock().unwrap()>0 {
            input_queue.dequeue();
            let mut num = counter.lock().unwrap();
            *num -= 1;  
        }



}






let s: Vec<_> = handles.drain(..).collect();

for handlee in s {
    let _x=handlee.join().unwrap();
}
    
for e in rx.try_iter() {
    print!("{} \n",e.result);
}


print!("\n");


}
print!("\n");
}