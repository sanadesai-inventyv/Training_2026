use lazy_static::lazy_static;
use std::sync::Mutex;

// 2. Static variable to track total requests
lazy_static!{
static ref TOTAL_REQUESTS: Mutex<u32> = Mutex::new(0);
}

// 1. Define enum Request
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

// 3. Handle request
fn handle_request(req: Request) -> String {
    // 3.2 Increment static counter
    let mut count = TOTAL_REQUESTS.lock().unwrap();
    *count += 1;

    // 3.1 Pattern matching
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

fn main() {
    // 4.1 Create requests
    let r1 = Request::Get {
        endpoint: "/home".to_string(),
    };

    let r2 = Request::Post {
        endpoint: "/login".to_string(),
        payload_size: 212,
    };

    let r3 = Request::Delete(5);

    // 4.2 Process them
    let res1 = handle_request(r1);
    let res2 = handle_request(r2);
    let res3 = handle_request(r3);

    // 4.3 Print responses
    println!("{}", res1);
    println!("{}", res2);
    println!("{}", res3);

    // Print total request count
    let total = TOTAL_REQUESTS.lock().unwrap();
    println!("Total requests processed: {}", *total);
}
