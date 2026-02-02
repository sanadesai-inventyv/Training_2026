use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
static ref TOTAL_REQUESTS: Mutex<u32> = Mutex::new(0);
}

enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

fn handle_request(req: Request) -> String {
    
    let mut count = TOTAL_REQUESTS.lock().unwrap();
    *count += 1;

    match req {
        Request::Get { endpoint } => {
            format!("GET request to {}", endpoint)
        }

        Request::Post {
            endpoint,
            payload_size,
        } => {
            format!("POST request to {} with payload {} bytes",
                endpoint, payload_size)
        }

        Request::Delete(id) => {
            format!("DELETE request for resource id {}", id)
        }
    }
}

pub fn run() {
    let r1 = Request::Get {
        endpoint: "/home".to_string(),
    };

    let r2 = Request::Post {
        endpoint: "/login".to_string(),
        payload_size: 212,
    };

    let r3 = Request::Delete(5);

    let res1 = handle_request(r1);
    let res2 = handle_request(r2);
    let res3 = handle_request(r3);

    println!("{}", res1);
    println!("{}", res2);
    println!("{}", res3);

    let total = TOTAL_REQUESTS.lock().unwrap();
    println!("Total requests processed: {}", *total);
}
