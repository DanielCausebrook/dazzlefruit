use std::collections::LinkedList;
use std::future::Future;
use std::io::{Error, ErrorKind};
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll};
use std::time::Duration;

use futures::task::AtomicWaker;
use tauri::async_runtime::{JoinHandle, spawn};
use tauri::{Emitter, Manager};
use tokio::io::{ReadBuf};
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;
use tokio::sync::RwLockWriteGuard;

use non_locking_io::{NonLockingRead, NonLockingWrite};
use packet_types::TcpPacketType;
use crate::{AppState, LockedAppState};

use crate::pico_connection::non_locking_io::NonLockingSend;
use crate::pico_connection::packet_types::UdpPacketType;
use crate::tauri_events::ConnectionOpenPayload;

pub mod packet_types;
pub mod non_locking_io;

struct PicoConnectionData {
    tcp_stream: RwLock<TcpStream>,
    udp_socket: RwLock<UdpSocket>,
    response_futures: Mutex<LinkedList<Arc<ResponseFutureInner>>>,
}

pub struct PicoConnection {
    data: Arc<PicoConnectionData>,
    incoming_data_handler: JoinHandle<()>,
}

impl Drop for PicoConnection {
    fn drop(&mut self) {
        self.incoming_data_handler.abort();
    }
}

pub type PicoConnectionHandle = Arc<PicoConnection>;

struct ResponseFutureInner {
    waker: AtomicWaker,
    response: Mutex<Option<Result<(), ()>>>,
}

#[derive(Clone)]
pub struct PicoConnectionResponseFuture(Arc<ResponseFutureInner>);

impl Future for PicoConnectionResponseFuture {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.response.lock().unwrap().take() {
            Some(result) => Poll::Ready(result),
            None => {
                self.0.waker.register(cx.waker());
                Poll::Pending
            }
        }
    }
}

impl PicoConnection {
    pub async fn new(ip: String, tcp_port: u16, udp_port: u16) -> Result<PicoConnectionHandle, String> {

        let tcp_stream = timeout(Duration::from_secs(10),TcpStream::connect((ip.clone(), tcp_port))).await
            .map_err(|_| format!("TCP connection timed out."))?
            .map_err(|e| format!("Failed to initialise TCP connection. ({})", e))?;

        let udp_socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| format!("Failed to bind UDP port locally. ({})", e))?;

        udp_socket.connect((ip.clone(), udp_port)).await
            .map_err(|e| format!("Failed to initialise UDP connection. ({})", e))?;

        let conn_data = Arc::new(PicoConnectionData {
            tcp_stream: RwLock::new(tcp_stream),
            udp_socket: RwLock::new(udp_socket),
            response_futures: Mutex::new(LinkedList::new()),
        });
        let conn = PicoConnection {
            data: conn_data.clone(),
            incoming_data_handler: spawn(handle_incoming_tcp_data(conn_data.clone())),
        };
        Ok(Arc::new(conn))
    }

    pub async fn send_tcp(&self, packet_type: TcpPacketType, data: &[u8]) -> std::io::Result<usize> {
        let full_data = [&[packet_type.into()], data].concat();
        self.data.tcp_stream.non_locking_write(&full_data).await
            .and_then(|bytes_sent| {
                if bytes_sent == 0 {
                    Err(Error::new(ErrorKind::WriteZero, "Connection closed."))
                } else {
                    Ok(bytes_sent)
                }
            })
    }

    async fn await_response(&self) -> Result<(), String> {
        let fut = PicoConnectionResponseFuture(Arc::new(ResponseFutureInner{
            waker: AtomicWaker::new(),
            response: Mutex::new(None),
        }));
        self.data.response_futures.lock().unwrap().push_back(fut.0.clone());
        timeout(Duration::from_secs(5), fut).await
            .map_err(|_| format!("Timed out waiting for server's response."))?
            .map_err(|_| format!("Server returned ERR."))
    }

    pub async fn send_tcp_await_response(&self, packet_type: TcpPacketType, data: &[u8]) -> std::io::Result<Result<(), String>> {
        self.send_tcp(packet_type, data).await?;
        Ok(self.await_response().await)
    }

    pub async fn send_udp(&self, packet_type: UdpPacketType, data: &[u8]) -> std::io::Result<usize> {
        let full_data = [&[packet_type.into()], data].concat();
        self.data.udp_socket.non_locking_send(&full_data).await
            .and_then(|bytes_sent| {
                if bytes_sent == 0 {
                    Err(Error::new(ErrorKind::WriteZero, "Connection closed."))
                } else {
                    Ok(bytes_sent)
                }
            })
    }
}

async fn handle_incoming_tcp_data(connection: Arc<PicoConnectionData>) {
    loop {
        let mut raw_buf = vec![0; 1];
        let mut buf = ReadBuf::new(&mut raw_buf);
        let read_result = connection.tcp_stream.non_locking_read(&mut buf).await;

        let packet_type = match read_result {
            Ok(()) => match buf.filled().first() {
                Some(x) => TcpPacketType::try_from(*x)
                    .expect(format!("Unknown packet type {} received.", *x).as_str()),
                None => return // Socket closed
            },
            Err(e) => panic!("Error in TCP socket: {}", e),
        };

        match packet_type {
            TcpPacketType::Ok => {
                let mut futures = connection.response_futures.lock().unwrap();
                for future in futures.split_off(0).iter_mut() {
                    future.response.lock().unwrap().replace(Ok(()));
                    future.waker.wake();
                }
            },
            TcpPacketType::Err => {
                let mut futures = connection.response_futures.lock().unwrap();
                for future in futures.split_off(0).iter_mut() {
                    future.response.lock().unwrap().replace(Err(()));
                    future.waker.wake();
                }
                println!("Received ERR from server.");
            },
            TcpPacketType::Ping => {
                connection.tcp_stream.non_locking_write(&[TcpPacketType::Ping.into()]).await
                    .expect("Ping response failed.");
            }
            _ => {}
        }
    }
}

#[tauri::command]
pub async fn connect(ip: String, tcp_port: u16, udp_port: u16, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;

    if state.connection.is_some() {
        Err(format!("Already connected to a Pico!"))
    } else {
        let conn = PicoConnection::new(ip.clone(), tcp_port, udp_port).await?;
        state.connection = Some(conn);
        state.app_handle.emit("connection-open", ConnectionOpenPayload{ ip }).unwrap();
        Ok(())
    }
}

#[tauri::command]
pub async fn disconnect(tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    state.neopixel_controller.take();
    state.connection.take().ok_or("Not connected to a pico!")?;
    state.app_handle.emit("connection-close", {}).unwrap();
    Ok(())
}
