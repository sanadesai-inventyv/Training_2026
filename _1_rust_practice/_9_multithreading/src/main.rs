use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,  
    threadId: String,
}

fn main() {
    let data = Arc::new(RwLock::new(Vec::<MultiThread>::new()));
    let counter = Arc::new(AtomicI32::new(1));
    let mut t1 = vec![];

    
    {
        let data = Arc::clone(&data);
        let counter = Arc::clone(&counter);

        let t2 = thread::spawn(move || {
            loop {
                let id_value = counter.fetch_add(1, Ordering::SeqCst);

                let now_secs = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let record = MultiThread {
                    id: id_value,
                    recordAddedTime: now_secs.to_string(),
                    threadId: "thread1".to_string(),
                };

                {
                    let mut d = data.write().unwrap();
                    d.push(record);
                }

                println!("Record Added {}", id_value);
                thread::sleep(Duration::from_secs(10));
            }
        });

        t1.push(t2);
    }

    
    {
        let data = Arc::clone(&data);

        let t2 = thread::spawn(move || {
            loop {
                {
                    let d = data.read().unwrap();
                    println!("current records: {:?}", *d);
                }
                thread::sleep(Duration::from_secs(3)); 
            }
        });

        t1.push(t2);
    }

    
    {
        let data = Arc::clone(&data);

        let t2 = thread::spawn(move || {
            loop {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                {
                    let mut d = data.write().unwrap();
                    d.retain(|r| {
                        let record_time = r.recordAddedTime.parse::<u64>().unwrap();
                        !(r.id % 2 == 0 && now - record_time > 20)
                    });
                }

                thread::sleep(Duration::from_secs(5));
            }
        });

        t1.push(t2);
    }

    
    {
        let data = Arc::clone(&data);

        let t2 = thread::spawn(move || {
            loop {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                {
                    let mut d = data.write().unwrap();
                    d.retain(|r| {
                        let record_time = r.recordAddedTime.parse::<u64>().unwrap();
                        !(r.id % 2 != 0 && now - record_time > 20)
                    });
                }

                thread::sleep(Duration::from_secs(5));
            }
        });

        t1.push(t2);
    }

    
    {
        let data = Arc::clone(&data);

        let t2 = thread::spawn(move || {
            loop {
                let d = data.read().unwrap();
                let count = d.iter().filter(|r| r.id % 2 == 0).count();
                println!("even count {}", count);
                thread::sleep(Duration::from_secs(5));
            }
        });

        t1.push(t2);
    }


    {
        let data = Arc::clone(&data);

        let t2 = thread::spawn(move || {
            loop {
                let d = data.read().unwrap();
                let count = d.iter().filter(|r| r.id % 2 != 0).count();
                println!("odd count: {}", count);
                thread::sleep(Duration::from_secs(5));
            }
        });

        t1.push(t2);
    }

    for h in t1 {
        h.join().unwrap();
    }
}
