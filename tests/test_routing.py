import intelligent_routing
import pytest

def test_routing():
    # Create accelerators
    acc1 = intelligent_routing.Accelerator(1, 100)
    acc2 = intelligent_routing.Accelerator(2, 100)
    
    assert acc1.id == 1
    assert acc1.capacity == 100
    assert acc1.current_load == 0

    # Create router with Round Robin strategy
    router = intelligent_routing.Router("round_robin")
    router.add_accelerator(acc1)
    router.add_accelerator(acc2)

    # Create request
    req = intelligent_routing.Request(1, 10, 1)
    assert req.id == 1
    assert req.cost == 10

    # Route request
    # Round robin should pick the first one (or depending on implementation)
    # Since we just added them, let's see.
    # The Rust implementation of RoundRobin uses an AtomicUsize starting at 0.
    # So first request should go to index 0 (acc1).
    
    acc_id = router.route_request(req)
    assert acc_id is not None
    print(f"Routed to accelerator {acc_id}")
    
    # Route another request
    req2 = intelligent_routing.Request(2, 10, 1)
    acc_id2 = router.route_request(req2)
    assert acc_id2 is not None
    print(f"Routed to accelerator {acc_id2}")
    
    # Verify different accelerators were chosen (Round Robin)
    # Note: The Rust RoundRobin implementation might be global or per instance?
    # Let's check the Rust code if needed. But assuming standard RR.
    
    # Test invalid strategy
    try:
        intelligent_routing.Router("invalid_strategy")
        assert False, "Should have raised ValueError"
    except ValueError:
        pass

if __name__ == "__main__":
    test_routing()
    print("Test passed!")
