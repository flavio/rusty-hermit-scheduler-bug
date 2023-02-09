#[cfg(target_os = "hermit")]
use hermit_sys as _;

use anyhow::Result;
use redis::Commands;
use redis::ConnectionLike;
use std::thread;

mod cli;

fn no_connection_pool(redis_host: &str) -> Result<()> {
    println!("running without connection pool");
    let client =
        redis::Client::open(format!("redis://{}/", redis_host)).expect("error opening connection");

    let mut con = client.get_connection()?;
    println!("check connection: {:?}", con.check_connection());
    let _ = con.set("my_key", "ciao")?;
    let value: String = con.get("my_key")?;
    println!("key set: {:?}", value);
    Ok(())
}

fn connection_pool(redis_host: &str) -> Result<()> {
    println!("running WITH connection pool");
    println!("creating client");
    let client =
        redis::Client::open(format!("redis://{}/", redis_host)).expect("error opening connection");
    println!("client created");

    println!("building pool - start");
    let pool = r2d2::Pool::builder().max_size(1).build(client)?;
    println!("building pool - done");

    let worker_pool_size = 2;
    for i in 0..worker_pool_size {
        let pool = pool.clone();
        let join_handle = thread::spawn(move || {
            println!("thread {} - trying to get connection", i);
            let con = pool.get();
            match con {
                Ok(mut c) => {
                    println!("thread {} - got connection", i);
                    // throw away the result, just make sure it does not fail
                    let _: () = c
                        .append("my_key2", format!("worker {}", i))
                        .expect("worker {i} cannot perform append operation");
                }
                Err(e) => {
                    print!("cannot get connection: {:?}", e)
                }
            };
            // read back the key and return it.  Because the return value
            // from the function is a result for integer this will automatically
            // convert into one.
        });
        join_handle.join().expect("error joining thread");
    }

    let value: Vec<String> = pool.get().unwrap().get("my_key2")?;
    println!("key set: {}", value.join(", "));

    Ok(())
}

fn main() -> Result<()> {
    let settings = match cli::parse_cli()? {
        Some(s) => s,
        None => return Ok(()),
    };

    if settings.enable_connection_pool {
        connection_pool(&settings.redis_host)
    } else {
        no_connection_pool(&settings.redis_host)
    }
}
