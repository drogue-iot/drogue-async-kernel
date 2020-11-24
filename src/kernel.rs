use crate::event::Optional;

pub trait Kernel {
    type Event: Optional;

    fn dispatch_event(event: Self::Event);
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
                    KERNEL.replace(kernel);
                    $kernel::get().dispatch($crate::Event::Kernel($crate::KernelEvent::Initialize));
                    // loop once to allow the initialization to occur prior to enabling interrupts.
                    $kernel::event_loop();
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
                KERNEL.as_mut().expect("unable to obtain kernel")
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
                while let Some(event) = self.event_queue.dequeue() {
                    match event {
                        $crate::Event::Kernel(kernel_event) => {
                            $(
                                self.$name.process( $crate::Event::Kernel(kernel_event));
                            )*
                        }
                        $crate::Event::Actor(ref app_event) => {
                            if ! app_event.is_none() {
                                $(
                                    if let Some(actor_event) = app_event.into() {
                                        self.$name.process( $crate::Event::Actor(actor_event) );
                                    }
                                )*
                            }
                        }
                    }
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

            fn dispatch_event(event: Self::Event) {
                if event.is_none() {
                    return
                }
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
