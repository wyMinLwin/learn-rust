mod greetings;

use greetings::en;
use greetings::fr;

fn main() {
    en::say_hello();
    fr::say_hello();
}
