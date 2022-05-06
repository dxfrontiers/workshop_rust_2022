
fn main() {
    sample_threads();
}


fn sample_threads(){
    let mut handles = Vec::new();

    for i in 0..4 {
        handles.push(thread::spawn(move || {
            for _ in 0..10{
                println!(" Thread {} hier!",i);
            }
        }));
    }
    handles.into_iter().for_each(|h|h.join().unwrap());
    println!("Threads done");
}

