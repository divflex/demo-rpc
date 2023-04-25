

#![allow(unused_variables, unused_imports)]

use anyhow::Ok;

use futures::{future, prelude::*};
use server::HelloServer;
use tarpc::{
    context,
    server::{incoming::Incoming, BaseChannel},
    tokio_serde::formats::Json,
};
use tokio::{
    main, select,
    sync::mpsc::{self, channel, Receiver, Sender},
    time::{self, Duration},
};

mod server;

/// can not be cloned
pub struct SingletonInstance {

}

impl SingletonInstance {
    pub async fn say_hi() -> String {
        return String::new()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 

    let (tx, rx) = mpsc::channel::<String>(10);

    let server = HelloServer { name: String::new(), tx };
    let server_handle = server.start();

    server_handle.await?;

    Ok(())
}
