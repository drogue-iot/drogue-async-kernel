use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;
use crate::actor::EventDrivenActor;
use async_embedded::unsync::Channel;
use crate::event::Event;
use async_embedded::task;


pub struct Delay {
    taco: bool,
}

pub struct DelayExpiration;

impl EventDrivenActor for Delay {
    type Event = DelayExpiration;

    fn start(&'static mut self, channel: &'static Channel<Event<Self::Event>>) {
        task::spawn(async move {
            loop {
                let event = channel.recv().await;
                match event {
                    Event::Actor(DelayExpiration) => {}
                    _ => {}
                }
            }
        });
    }
}

impl Delay {
    pub async fn delay(ms: u8) {
        struct F {
            expired: bool,
        }

        impl F {
            pub(crate) fn has_expired(&self) -> bool {
                self.expired
            }
        }

        impl Future for F {
            type Output = ();

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                if self.has_expired() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }

        F {
            expired: false,
        }.await
    }
}
