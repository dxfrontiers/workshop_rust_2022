
#[cfg(test)]
mod async_basics{
    use tokio::{task, io};
    use std::{thread, time};
    use tokio::runtime::Runtime;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::error::Error;

    

    #[tokio::test]
    async fn basics_task_needs_polling(){
        println!("A");
        task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_secs(1));
        println!("C");
    }




    #[tokio::test]
    async fn basics_task_is_polled(){
        println!("A");
        let t = task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_secs(1));
        let _ = t.await;
        thread::sleep(time::Duration::from_secs(1));
        println!("C");
    }

    

    #[test]
    fn schedule_from_outside(){
        let my_task = async {
            println!("hello ");
            task::spawn(async {println!("world");});
            println!("!");
        };
        let rt = Runtime::new().unwrap();
        rt.spawn(my_task);
        // a normal application would either run forever or wait until the runtime becomes idle
        thread::sleep(time::Duration::from_secs(1));
    }

   


    // #[tokio::test]
    async fn echo() -> Result<(), Box<dyn Error>>  {
        let listener = TcpListener::bind(&"127.0.0.1:9000").await?;
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(handle_connection(stream));
        }
    }
    
    async fn handle_connection(mut stream: TcpStream) -> io::Result<u32>{
        println!("Handling conncection");
        let mut buf = vec![0; 1024];
        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 { break; }
            println!("Read {} bytes",n);

            stream.write_all(&buf[0..n]).await?;
            // stream.write_all(&buf[0..n]); // auch nett
        }
        println!("Connection closed");
        io::Result::Ok(0)
    }
}