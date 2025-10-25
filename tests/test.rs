use htils::{debug, ifsome};

#[test]
fn testy() {
    
    let r = "ddd".parse::<i32>();
    
    ifsome!(r.err(), |e| debug!(e));
    
}