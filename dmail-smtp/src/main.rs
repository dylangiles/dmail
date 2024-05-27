// use async_std::stream::StreamExt;
use chumsky::Parser;
use futures::StreamExt;
use server::SmtpSession;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::prelude::*;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal::ctrl_c;
use tokio::spawn;
use tokio::sync::broadcast;
// use std::net::TcpStream;

mod config;
mod server;
mod smtp;
mod smtp_parser;

use crate::config::Config;
use crate::smtp_parser::parser;
use crate::smtp_parser::Command;

pub(crate) const CRLF: &str = "\r\n";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::from_file(std::path::Path::new("./dmail.toml"))?;
    // let config = Config::default();
    println!("{config:#?}");

    // Listen for incoming TCP connections on localhost port 7878

    let server = TcpListener::bind(format!("0.0.0.0:{}", config.smtp.port))
        .await
        .expect("could not bind to the port");
    let (quit_tx, quit_rx) = broadcast::channel::<()>(1);

    println!("Listening on port {}", config.smtp.port);
    loop {
        tokio::select! {
            Ok(_) = ctrl_c() => {
                println!("Server interrupted. Gracefully shutting down.");
                quit_tx.send(()).unwrap();
                // quit_tx.send(()).context("failed to send quit signal").unwrap();
                break;
            }
            Ok((socket, _)) = server.accept() => {
                // spawn(session::handle_user_session(Arc::clone(&room_manager), quit_rx.resubscribe(), socket));
                spawn(session(quit_rx.resubscribe(), socket));
            }
        }
    }

    Ok(())
}

async fn session(mut quit_rx: broadcast::Receiver<()>, mut stream: TcpStream) {
    let this_session = SmtpSession::new();

    reply("220 localhost ESMTP dmail", &mut stream)
        .await
        .unwrap();

    let (mut tcp_reader, mut writer) = stream.into_split();

    loop {
        let mut buf = [0; 1024];
        let temp_str = std::str::from_utf8(&buf).unwrap();
        let mut buf_reader = BufReader::new(&mut tcp_reader);
        let num_bytes = match buf_reader.read(&mut buf).await {
            Ok(n) => n,
            Err(err) => {
                println!("{err:#?}");
                break;
            }
        };

        let (commands, errors) = parser()
            .parse(std::str::from_utf8(&buf).unwrap())
            .into_output_errors();

        println!("{commands:#?}");

        match commands {
            Some(cmd) => match cmd {
                Command::Helo(host) => {
                    let _ = reply(format!("250 Nice to meet you, {host}."), &mut writer).await;
                }

                Command::Quit => {
                    let reply_bytes = reply("221 Bye", &mut writer).await;
                    break;
                }
            },
            None => {
                let _ = reply("Unknown command", &mut writer).await;
                println!("{errors:#?}");
            }
        }
    }
}

async fn reply<A, W>(msg: A, stream: &mut W) -> Result<usize, std::io::Error>
where
    A: AsRef<str> + Display,
    W: AsyncWrite + std::marker::Unpin,
{
    stream.write(format!("{msg}\r\n").as_bytes()).await
}
