// use std::collections::BTreeMap;
//
//
// pub struct Subject<E>
//     where E: PartialOrd + Ord
// {
//     observers: Vec::<Box<dyn Fn(E)>>,
// }
//
//
// impl<E> Subject<E>
//     where E: PartialOrd + Ord
// {
//     pub fn new() -> Subject<E> {
//         Subject {
//             observers: Vec::new(),
//         }
//     }
//
//     pub fn attach_callback<F>(&mut self, callback: F)
//                               where F: 'static + Fn(&E),
//     {
//         self.observers.insert(Box::new(callback));
//     }
//
//     pub fn detach_callback<F>(&mut self, callback: F)
//                               where F: 'static + Fn(&E),
//     {
//         self.observers.insert(Box::new(callback));
//     }
//
//     pub fn do_callbacks<E>(&self, event: E) {
//         for callback in &self.subscribers {
//             callback(&event);
//         }
//     }
// }