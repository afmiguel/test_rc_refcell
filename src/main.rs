use std::rc::Rc;
use std::cell::RefCell;

// Our simpler data structure to be shared and mutated
struct SharedData {
    id: String,
    value: i32, // A mutable piece of data
}

impl SharedData {
    fn new(id: &str, initial_value: i32) -> Self {
        SharedData {
            id: id.to_string(),
            value: initial_value,
        }
    }

    fn update_value(&mut self, new_value: i32) {
        println!("Updating value for '{}' from {} to {}", self.id, self.value, new_value);
        self.value = new_value;
    }

    fn increment_value(&mut self) {
        println!("Incrementing value for '{}' from {} to {}", self.id, self.value, self.value + 1);
        self.value += 1;
    }

    fn display(&self) {
        println!("Data ID: {}, Current Value: {}", self.id, self.value);
    }
}

fn main() {
    // Main Moment 1: Initial setup.
    // Create an instance of SharedData wrapped in RefCell (for interior mutability)
    // and then wrapped in Rc (for shared ownership).
    let shared_item = Rc::new(RefCell::new(SharedData::new("ConfigItem", 10)));

    // Main Moment 2: Observing initial state.
    // Print the initial state of the shared data and its reference count.
    // .borrow() is used for immutable access to the data inside RefCell.
    println!("--- Initial State ---");
    println!("Initial strong count: {}", Rc::strong_count(&shared_item));
    shared_item.borrow().display();

    // Main Moment 3: Component A gets shared access.
    // Clone the Rc to give Component A shared ownership.
    // The strong count increases. Component A reads the current state.
    println!("\n--- Component A (gets read access) ---");
    let component_a_ref = Rc::clone(&shared_item);
    println!("Strong count after Component A clone: {}", Rc::strong_count(&shared_item));
    component_a_ref.borrow().display();

    // Main Moment 4: Component B gets shared access.
    // Clone the Rc again for Component B.
    // The strong count increases further.
    println!("\n--- Component B (gets write access and modifies) ---");
    let component_b_ref = Rc::clone(&shared_item);
    println!("Strong count after Component B clone: {}", Rc::strong_count(&shared_item));

    // Main Moment 5: Component B modifies the shared data.
    // .borrow_mut() is used for mutable access. This call will panic if
    // borrowing rules are violated (e.g., another mutable borrow is active).
    component_b_ref.borrow_mut().update_value(25);
    component_b_ref.borrow_mut().increment_value(); // Value becomes 26

    // Main Moment 6: Component B confirms its modifications.
    // Display the data from Component B's perspective.
    println!("Component B finished modifications.");
    component_b_ref.borrow().display();

    // Main Moment 7: Component A observes the changes.
    // Component A's reference now sees the data modified by Component B,
    // demonstrating that they share the same underlying data.
    println!("\n--- Component A (reads again) ---");
    println!("Component A sees updated data:");
    component_a_ref.borrow().display();

    // Main Moment 8: Original reference also observes changes.
    // The original 'shared_item' reference also sees the updated data.
    // Display the final state and reference count before any Rc instances are dropped.
    println!("\n--- Original Reference (reads again) ---");
    println!("Main's 'shared_item' sees updated data:");
    shared_item.borrow().display();
    println!("Final strong count before drops: {}", Rc::strong_count(&shared_item));

    // Main Moment 9: Automatic cleanup.
    // As component_a_ref, component_b_ref, and shared_item go out of scope at the end of main,
    // their destructors are called. The Rc strong count decreases for each.
    // When the strong count reaches zero, the RefCell and the SharedData it contains are dropped,
    // freeing the memory. This happens automatically.
}