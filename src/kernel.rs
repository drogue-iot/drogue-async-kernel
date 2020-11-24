pub trait Kernel {
    type Event;

    fn dispatch(event: Self::Event);
}

#[macro_export]
macro_rules! kernel {
    ( $kernel:ident<$event:ty> { $($name:ident: $ty:path $( => $irq:path )? ),* $(,)?} ) => {
        static mut KERNEL: Option<$kernel> = None;
        pub struct $kernel {
            event_queue: $crate::heapless::spsc::Queue<$crate::event::Event<$event>, $crate::heapless::consts::U16>,
            $(
                $name: $ty,
            )*
        }

        impl $kernel {
            pub fn start(
                $(
                    $name: $ty,
                )*
            ) -> ! {
                let mut kernel = Self {
                    event_queue: $crate::heapless::spsc::Queue::new(),
                    $(
                        $name,
                    )*
                };

                use $crate::actor::InterruptHandler;

                unsafe {
                    $(
                        kernel.$name.process(
                            $crate::Event::Kernel($crate::KernelEvent::Initialize)
                        );

                    )*
                    KERNEL.replace(kernel);
                    $(
                        $(
                            cortex_m::peripheral::NVIC::unmask( <$irq>::IRQ );
                        )?
                    )*
                }

                loop {
                    $kernel::event_loop();
                }
            }

            unsafe fn get() -> &'static mut Self {
                KERNEL.as_mut().unwrap()
            }

            $(
                pub fn $name() -> &'static mut $ty {
                    unsafe { &mut Self::get().$name }
                }
            )*

            fn dispatch(&mut self, event: $crate::event::Event<$event>) {
                self.event_queue.enqueue(event).ok();
            }

            fn run_event_loop(&mut self) {
                while let Some($crate::event::Event::Actor(ref event)) = self.event_queue.dequeue() {
                    $(
                        if let Some(actor_event) = event.into() {
                            self.$name.process( $crate::event::Event::Actor(actor_event) );
                        }
                    )*
                }
            }

            fn event_loop() {
                unsafe {
                    Self::get().run_event_loop();
                }
            }
        }


        impl $crate::kernel::Kernel for $kernel {
            type Event = $event;

            fn dispatch(event: Self::Event) {
                unsafe {
                    Self::get().dispatch($crate::event::Event::Actor(event));
                }
            }
        }

        #[exception]
        unsafe fn DefaultHandler(arg: i16) {
            use cortex_m::interrupt::Nr;
            use $crate::actor::InterruptHandler;
            $(
                $(
                    if arg == <$irq as InterruptHandler<_, _>>::IRQ.nr() as i16 {
                      if ( <$irq as InterruptHandler<_,_>>::check_interrupt( $kernel::$name() ) ) {
                          $kernel::$name().interrupt();
                          <$irq as InterruptHandler<_,_>>::clear_interrupt( $kernel::$name() );
                      }
                    }
                )?
            )*
        }
    }
}