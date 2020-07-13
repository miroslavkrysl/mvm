use crate::vm::memory::Heap;
use crate::vm::exec::class_loader::ClassLoader;

pub struct Runtime {
    heap: Heap,
    class_loader: ClassLoader
}