use crate::ClipBox;
use std::sync::{Arc, Mutex};

// SharedState is used to share data between threads
pub struct SharedState {
    // using Arc<Mutex<>> to share data between threads
    // requires more memory than Rc<RefCell<>> but is thread safe
    pub clip_box: Arc<Mutex<ClipBox>>,
}
impl SharedState {
    pub fn new(clip_box: ClipBox) -> Self {
        SharedState {
            clip_box: Arc::new(Mutex::new(clip_box)),
        }
    }
}
