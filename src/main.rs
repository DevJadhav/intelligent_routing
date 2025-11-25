use intelligent_routing::accelerator::Accelerator;
use intelligent_routing::request::Request;
use intelligent_routing::router::Router;
use intelligent_routing::strategies::{least_connections::LeastConnections, p2c::PowerOfTwoChoices, round_robin::RoundRobin};
use rand::Rng;
use std::time::Instant;

fn main() {
    println!("Starting Intelligent Routing Simulation...");

    // 1. Setup Accelerators
    let num_accelerators = 10000;
    let mut accelerators = Vec::with_capacity(num_accelerators);
    for i in 0..num_accelerators {
        accelerators.push(Accelerator::new(i, 100)); // Capacity 100
    }

    // 2. Setup Router with a Strategy
    // let strategy = Box::new(RoundRobin::new());
    // let strategy = Box::new(LeastConnections::new());
    let strategy = Box::new(PowerOfTwoChoices::new());
    
    let mut router = Router::new(strategy);
    for acc in accelerators {
        router.add_accelerator(acc);
    }

    // 3. Generate Requests
    let num_requests = 100000;
    let mut rng = rand::thread_rng();
    
    let start_time = Instant::now();
    let mut success_count = 0;
    let mut fail_count = 0;

    for i in 0..num_requests {
        let cost = rng.random_range(1..10);
        let req = Request::new(i, cost, 1);
        
        match router.route_request(&req) {
            Some(_acc_id) => {
                // println!("Request {} routed to Accelerator {}", i, acc_id);
                success_count += 1;
            }
            None => {
                // println!("Request {} failed to route", i);
                fail_count += 1;
            }
        }
        
        // Simulate load decay occasionally to free up space
        if i % 100 == 0 {
             for acc in &mut router.accelerators {
                 acc.remove_load(5); // Decay load
             }
        }
    }

    let duration = start_time.elapsed();

    println!("Simulation Complete!");
    println!("Time elapsed: {:?}", duration);
    println!("Total Requests: {}", num_requests);
    println!("Successful Routes: {}", success_count);
    println!("Failed Routes: {}", fail_count);
    
    // Calculate load distribution stats
    let loads: Vec<u32> = router.accelerators.iter().map(|a| a.current_load).collect();
    let total_load: u32 = loads.iter().sum();
    let avg_load = total_load as f64 / num_accelerators as f64;
    
    // Variance
    let variance: f64 = loads.iter()
        .map(|&load| {
            let diff = load as f64 - avg_load;
            diff * diff
        })
        .sum::<f64>() / num_accelerators as f64;
    let std_dev = variance.sqrt();

    println!("Average Load: {:.2}", avg_load);
    println!("Load Std Dev: {:.2}", std_dev);
}
