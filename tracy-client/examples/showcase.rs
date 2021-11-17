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

        let mut query_pool = [0_i64; 9];
        let mut head = 0;
        let mut tail = 0;

        for _ in 0 .. 1000 {
            {
                let start_query_id = tail;
                tail = (tail + 1) % query_pool.len();
                let end_query_id = tail;
                tail = (tail + 1) % query_pool.len();
                let _gpu_zone = gpu_zone!("testtt", start_query_id as u16, end_query_id as u16, 0);

                query_pool[start_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

                std::thread::sleep(std::time::Duration::from_millis(1));

                query_pool[end_query_id] = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;
            }

            std::thread::sleep(std::time::Duration::from_millis(4));

            while head != tail {
                emit_gpu_time(query_pool[head], 0, head as u16);

                head = (head + 1) % query_pool.len();
            }

            std::thread::sleep(std::time::Duration::from_millis(4));

            finish_continuous_frame!();
        }
    }
    message("finished gpu stuff", 10);
}
