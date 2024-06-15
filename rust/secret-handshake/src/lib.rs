const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = Vec::new();
    for (i, action) in ACTIONS.into_iter().enumerate() {
        if n & 1 << i != 0 {
            actions.push(action);
        }
    }
    if n & 1 << 4 != 0 {
        actions.reverse();
    }
    actions
}
