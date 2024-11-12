use jetgpio_sys::{gpioInitialise, gpioSetMode, gpioTerminate, gpioWrite, JET_OUTPUT};

fn with_error_check(result: i32) -> Result<(), i32> {
    if result < 0 {
        return Err(result);
    }
    return Ok(());
}

// This example blinks an LED connected to pin 3.
// The LED should blink 10 times.
// Make sure to include a resistor in series with the LED.
fn main() {
    with_error_check(unsafe { gpioInitialise() }).expect("Failed to initialize GPIO");

    // set pin 3 as output
    with_error_check(unsafe { gpioSetMode(3, JET_OUTPUT) }).expect("Failed to set pin 3 as output");

    // blink 10 times
    for _ in 0..10 {
        // set pin 3 to high
        with_error_check(unsafe { gpioWrite(3, 1) }).expect("Failed to set pin 3 to high");
        std::thread::sleep(std::time::Duration::from_millis(500));

        // set pin 3 to low
        with_error_check(unsafe { gpioWrite(3, 0) }).expect("Failed to set pin 3 to low");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // teardown
    unsafe { gpioTerminate() };
}
