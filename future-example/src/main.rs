extern crate futures;
extern crate futures_cpupool;

use std::io;
use futures::Future;
use futures_cpupool::CpuPool;

fn check_prime_boxed(n: u64) -> Box<dyn Future<Item = bool, Error = io::Error>> {
    for i in 2..n {
        if n % i == 0 {
            return Box::new(futures::future::ok(false))
        }
    }
    Box::new(futures::future::ok(true))
}

fn check_prime_impl_trait(n: u64) -> impl Future<Item = bool, Error = io::Error> {
    for i in 2..n {
        if n % i == 0 {
            return futures::future::ok(false)
        }
    }
    futures::future::ok(true)
}

fn check_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    true
}



fn main() {
    let input: u64 = 58466453;
    //let input: u64 = 100;
    let res_one = check_prime_boxed(input);
    //let res_two = check_prime_impl_trait(58466453);
    println!("111");
    //Block the current thread until this future is resolved.
    println!("{}", res_one.wait().unwrap());
    //println!("{}", res_two.wait().unwrap());
    println!("222");

    let thread_pool = CpuPool::new(4);
    let res_three = thread_pool.spawn_fn(move || {
        let temp = check_prime(58466453);
        let result: Result<bool, ()> = Ok(temp);
        result
    });
    println!("{} {}", 1234,res_three.wait().unwrap());
    //println!("{}", res_one.wait().unwrap());
}
