use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
	let mut leftover = VecDeque::new();
	// Adding the following line produces an ICE
	let item = ();
	// Here I'd expect diagnostic à la "help: you might have meant to use pattern matching"
	// It is produced as expected, if the line above is commented.
        // The "let" here is commented, because adding it stops the ICE from appearing.
	while /*let*/ Some(item) = leftover.pop_back() {
		queue.push_front(item)
	}
}
