#[cfg(test)]
mod test{
    use std::{error::Error, time::Duration, thread};

    // #[tokio::test]
    async fn many() -> Result<(), Box<dyn Error>>  {
        for i in 0..20_000_000 {
            tokio::spawn( async move {
                tokio::time::sleep(Duration::from_secs(20)).await;
            });
            if i%1_000_000 == 0 {
                println!("spawned {} tasks", i)
            }
        }

        println!("Done spawning");
        thread::sleep(Duration::from_secs(30));

        Ok(())
    }
}