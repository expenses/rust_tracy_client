use std::thread::{sleep, spawn};
use tracy_client::*;
use tracy_client_sys::*;

#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

fn fib(i: u16) -> u64 {
    let span = Span::new(&format!("fib({})", i), "fib", file!(), line!(), 100);
    let result = match i {
        0 => 0,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    };
    span.emit_value(result);
    result
}

fn main() {
    message("trying gpu stuff", 10);
    unsafe {
        let gpu_time = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

        dbg!(gpu_time);

        emit_new_gpu_context(gpu_time, 1.0, 0, GpuContextType::Vulkan, None);

        std::thread::sleep(std::time::Duration::from_secs(1));

        struct TimestampBuffer<const N: usize> {
            timestamps: [i64; N],
            len: usize,
        }

        impl<const N: usize> Default for TimestampBuffer<N> {
            fn default() -> Self {
                Self {
                    timestamps: [0; N],
                    len: 0,
                }
            }
        }

        impl<const N: usize> TimestampBuffer<N> {
            fn emit(&mut self) {
                for i in 0 .. self.len {
                    emit_gpu_time(self.timestamps[i], 0, i as u16);
                }

                self.len = 0;
            }

            fn next_id(&mut self) -> usize {
                let next_id = self.len;
                self.len += 1;
                next_id
            }
        }

        let mut timestamps = TimestampBuffer::<63>::default();

        loop {
            {
                let start_query_id = timestamps.next_id();
                let end_query_id = timestamps.next_id();
                let _gpu_zone = gpu_zone!("testtt", start_query_id as u16, end_query_id as u16, 0);

                timestamps.timestamps[start_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

                std::thread::sleep(std::time::Duration::from_millis(1));

                timestamps.timestamps[end_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;
            }

            {
                let start_query_id = timestamps.next_id();
                let end_query_id = timestamps.next_id();
                let _gpu_zone = gpu_zone!("t2", start_query_id as u16, end_query_id as u16, 0);

                timestamps.timestamps[start_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

                std::thread::sleep(std::time::Duration::from_millis(1));

                {
                    let start_query_id = timestamps.next_id();
                    let end_query_id = timestamps.next_id();
                    let _gpu_zone = gpu_zone!("scoped", start_query_id as u16, end_query_id as u16, 0);

                    timestamps.timestamps[start_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

                    std::thread::sleep(std::time::Duration::from_nanos(500));

                    timestamps.timestamps[end_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;
                }

                std::thread::sleep(std::time::Duration::from_millis(1));

                timestamps.timestamps[end_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;
            }

            std::thread::sleep(std::time::Duration::from_millis(4));

            timestamps.emit();

            std::thread::sleep(std::time::Duration::from_millis(4));

            finish_continuous_frame!();
        }
    }
    message("finished gpu stuff", 10);
}
