
#[cfg(test)]
mod tests{
    use std::{thread::{self, JoinHandle}, time::Duration, sync::{Arc, atomic::{AtomicU64, Ordering}, Mutex}};

    #[test]
    fn sample_threads(){
        let mut handles = Vec::new();
    
        for i in 0..4 {
            handles.push(thread::spawn(move || {
                for _ in 0..10{
                    println!(" Thread {} hier!",i);
                }
            }));
        }
    
        thread::sleep(Duration::from_secs(2));
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("Threads done");
    }
    
    
    static DATA: [u32;3] = [0,1,2];
    #[test]
    fn compute_with_reference() {
        // let vec = vec![0u32; 4096];
        let mut handles = Vec::new();
    
        for i in 0..3 {
            let my_data = &DATA;
            handles.push(thread::spawn(move || {
                println!("[{:2}]: {}", i, my_data[i]);
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("{}", DATA[0]);
    }
    
    
    
    #[test]
    fn compute_with_arc() {
        let vec = vec![0u32; 4096];
        let data = Arc::new(vec);
        let mut handles = Vec::new();
    
        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                println!("[{:2}]: {}", i, data[i]);
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap())
    }
    
    
    
    #[test]
    fn atomic_arc() {
        let ctr = AtomicU64::new(0);
        let data = Arc::new(ctr);
    
        let mut handles = Vec::new();
    
        for _ in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                data.fetch_add(1, Ordering::SeqCst);
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("Result is {}",data.load(Ordering::SeqCst));
    }
    
    
    #[test]
    fn test_mutex() {
        let data = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();
    
        for _ in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                let mut guard = data.lock().expect("poisioned");
                *guard +=1;
                // no unlock required
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        let result = {
            *data.lock().unwrap()
        };
        println!("Result: {}",result);
        assert_eq!(result,16)
    }

    #[test]
    fn test_mutex_short() {
        let data = Arc::new(Mutex::new(0));
    
        let handles = (0..16)
            .map(|_| data.clone())
            .map(|arc| 
                thread::spawn( move || 
                    *arc.lock().expect("poisioned")+=1 
                )
            )
            .collect::<Vec<JoinHandle<()>>>(); // vorsicht mit lazy evaluation

        handles.into_iter().for_each(|h|h.join().unwrap());
        let result = *data.lock().unwrap();
        println!("Result: {}",result);
        assert_eq!(result,16)
    }



    
}