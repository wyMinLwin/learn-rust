#[derive(Debug)]
pub struct Batman {
    pub iq: u8,
}

impl Batman {
    pub fn fight(&self) {
        println!("Batman is fighting!");
    }
}
