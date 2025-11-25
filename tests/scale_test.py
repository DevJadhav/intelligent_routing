import intelligent_routing
import time
import random

def run_simulation(strategy_name, num_accelerators, num_requests):
    print(f"Testing {strategy_name} with {num_accelerators} accelerators and {num_requests} requests...")
    
    # Setup Accelerators
    accelerators = []
    for i in range(num_accelerators):
        accelerators.append(intelligent_routing.Accelerator(i, 100))
    
    # Setup Router
    router = intelligent_routing.Router(strategy_name)
    for acc in accelerators:
        router.add_accelerator(acc)
        
    # Generate and Route Requests
    start_time = time.time()
    success_count = 0
    
    for i in range(num_requests):
        cost = random.randint(1, 10)
        req = intelligent_routing.Request(i, cost, 1)
        
        if router.route_request(req) is not None:
            success_count += 1
            
    duration = time.time() - start_time
    print(f"Completed in {duration:.2f}s")
    print(f"Throughput: {num_requests / duration:.2f} req/s")
    print(f"Success Rate: {success_count / num_requests * 100:.2f}%")
    print("-" * 20)

if __name__ == "__main__":
    # Test P2C at scale
    run_simulation("p2c", 10000, 100000)
    
    # Test Round Robin at scale
    run_simulation("round_robin", 10000, 100000)
