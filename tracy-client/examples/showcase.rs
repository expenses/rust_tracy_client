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

        ___tracy_emit_gpu_new_context(___tracy_gpu_new_context_data {
            gpuTime: gpu_time,
            period: 1.0,
            context: 0,
            flags: 0,
            type_: 2,
        });

        let name = "My Vulkan Context";

        ___tracy_emit_gpu_context_name(___tracy_gpu_context_name_data {
            context: 0,
            name: name.as_ptr() as _,
            len: name.len() as _
        });

        std::thread::sleep(std::time::Duration::from_secs(1));

        let file = "file.rs";
        let function = "fn test";
        let name = "gpu test";
        let line = 10;

        let mut query_id = 0;

        for _ in 0 .. 1000 {
            let begin_query_id = query_id;

            query_id += 1;

            let end_query_id = query_id;

            query_id += 1;

            let loc = ___tracy_alloc_srcloc_name(
                line,
                file.as_ptr() as _,
                file.len(),
                function.as_ptr() as _,
                function.len(),
                name.as_ptr() as _,
                name.len(),
            );

            ___tracy_emit_gpu_zone_begin_alloc(___tracy_gpu_zone_begin_data {
                srcloc: loc,
                queryId: begin_query_id,
                context: 0,
            }, 1);

            let start_gpu_time = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

            std::thread::sleep(std::time::Duration::from_millis(1));

            let end_gpu_time = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % i64::max_value() as u128) as i64;

            ___tracy_emit_gpu_zone_end(___tracy_gpu_zone_end_data {
                context: 0,
                queryId: end_query_id
            }, 1);

            std::thread::sleep(std::time::Duration::from_millis(4));

            ___tracy_emit_gpu_time(___tracy_gpu_time_data {
                gpuTime: start_gpu_time,
                context: 0,
                queryId: begin_query_id,
            });

            ___tracy_emit_gpu_time(___tracy_gpu_time_data {
                gpuTime: end_gpu_time,
                context: 0,
                queryId: end_query_id,
            });

            std::thread::sleep(std::time::Duration::from_millis(4));

            finish_continuous_frame!();
        }
    }
    message("finished gpu stuff", 10);
}
