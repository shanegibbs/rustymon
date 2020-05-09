use log::{info, trace};

pub struct Status {

}

impl Status {
    pub fn new() -> Self {
        Status {}
    }

    pub fn handle(&self) {
        info!("handle")
    }
}
