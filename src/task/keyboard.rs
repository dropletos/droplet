use core::char;
use core::task::Context;

use crate::print;
use crate::println;
use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    future::Ready,
    stream::{Pending, Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use x86_64::structures::idt::InterruptStackFrame;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("advise: scancode que full, dropping keyboard")
        } else {
            WAKER.wake();
        }
    } else {
        println!("advise: scancode que uninitalised.")
    }
}

pub struct ScancodeStream {
    _priavte: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("scancodestream::new only called once dimwit");
        ScancodeStream { _priavte: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not inital");
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(char) => print!("{}", char),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}
