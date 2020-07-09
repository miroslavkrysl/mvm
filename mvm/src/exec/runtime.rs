use crate::memory::Heap;
use crate::exec::class_loader::ClassLoader;

pub struct Runtime {
    heap: Heap,
    class_loader: ClassLoader
}