// create a new processor and allocate a callback funtion into box content
// let mut processor = util::callback::Processor { callback: Box::new(simple_callback) };
//
// or replace give callbacks
// processor.set_callback(newCallback);
//
// execute callback
// processor.process_events();

pub struct Processor {
    // Boxed Callback, making the content static
    pub callback: Box<FnMut()>,
}

impl Processor {
    // replacing the boxed callback
    pub fn set_callback<CB: 'static + FnMut()>(&mut self, callback: CB) {
        self.callback = Box::new(callback);
    }

    // execute the callback
    pub fn process_events(&mut self) {
        (self.callback)();
    }
}