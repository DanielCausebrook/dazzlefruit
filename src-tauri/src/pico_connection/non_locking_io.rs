use std::{
    future::Future,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll}
};
use std::sync::RwLock;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{UdpSocket};

pub struct NonLockingReadFuture<'a, 'b, T: AsyncRead + Unpin> {
    locked_io: &'a RwLock<T>,
    buf: &'a mut ReadBuf<'b>,
}

impl<T: AsyncRead + Unpin> Future for NonLockingReadFuture<'_, '_, T> {
    type Output = std::io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(self.locked_io.write().unwrap().deref_mut()).poll_read(cx, self.buf)
        // match Box::pin(self.locked_io.write()).as_mut().poll(cx) {
        //     Poll::Ready(mut io) => {
        //         Pin::new(io.deref_mut()).poll_read(cx, self.buf)
        //     }
        //     Poll::Pending => Poll::Pending
        // }
    }
}

pub trait NonLockingRead<T: AsyncRead + Unpin> {
    fn non_locking_read<'a, 'b>(self: &'a Self, buf: &'a mut ReadBuf<'b>) -> NonLockingReadFuture<'a, 'b, T>;
}

impl<T: AsyncRead + Unpin> NonLockingRead<T> for RwLock<T> {
    fn non_locking_read<'a, 'b>(self: &'a Self, buf: &'a mut ReadBuf<'b>) -> NonLockingReadFuture<'a, 'b, T> {
        NonLockingReadFuture{ locked_io: &self, buf }
    }
}


pub struct NonLockingWriteFuture<'a, T: AsyncWrite + Unpin> {
    locked_io: &'a RwLock<T>,
    buf: &'a [u8],
}

impl<T: AsyncWrite + Unpin> Future for NonLockingWriteFuture<'_, T> {
    type Output = std::io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>,) -> Poll<Self::Output> {
        Pin::new(self.locked_io.write().unwrap().deref_mut()).poll_write(cx, self.buf)
        // match Box::pin(self.locked_io.write()).as_mut().poll(cx) {
        //     Poll::Ready(mut io) => {
        //         Pin::new(io.deref_mut()).poll_write(cx, self.buf)
        //     }
        //     Poll::Pending => Poll::Pending
        // }
    }
}

pub trait NonLockingWrite<T: AsyncWrite + Unpin> {
    fn non_locking_write<'a>(self: &'a Self, buf: &'a [u8]) -> NonLockingWriteFuture<'a, T>;
}

impl<T: AsyncWrite + Unpin> NonLockingWrite<T> for RwLock<T> {
    fn non_locking_write<'a>(self: &'a Self, buf: &'a [u8]) -> NonLockingWriteFuture<'a, T> {
        NonLockingWriteFuture{ locked_io: &self, buf }
    }
}


pub struct NonLockingSendFuture<'a> {
    locked_io: &'a RwLock<UdpSocket>,
    buf: &'a [u8],
}

impl Future for NonLockingSendFuture<'_> {
    type Output = std::io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>,) -> Poll<Self::Output> {
        Pin::new(self.locked_io.write().unwrap().deref_mut()).poll_send(cx, self.buf)
        // match Box::pin(self.locked_io.write()).as_mut().poll(cx) {
        //     Poll::Ready(mut io) => {
        //         Pin::new(io.deref_mut()).poll_send(cx, self.buf)
        //     }
        //     Poll::Pending => Poll::Pending
        // }
    }
}

pub trait NonLockingSend {
    fn non_locking_send<'a>(self: &'a Self, buf: &'a [u8]) -> NonLockingSendFuture<'a>;
}

impl NonLockingSend for RwLock<UdpSocket> {
    fn non_locking_send<'a>(self: &'a Self, buf: &'a [u8]) -> NonLockingSendFuture<'a> {
        NonLockingSendFuture{ locked_io: &self, buf }
    }
}
