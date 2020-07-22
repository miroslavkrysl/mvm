use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::vm::class::class::Class;
use crate::vm::class::name::ClassName;
use crate::vm::exec::error::{ClassLoadError, ClassLoadErrorKind};
use crate::vm::parse::parser::ClassFileParser;


/// A class loader which can search for
/// class definitions in the filesystem and
/// load them.
pub struct ClassLoader {
    paths: Vec<PathBuf>
}


impl ClassLoader {
    /// Creates a new class loader with given class paths
    pub fn new(paths: Vec<PathBuf>) -> Self {
        ClassLoader {
            paths
        }
    }

    /// Loads class of the given name.
    /// Searches in the defined class paths until it find the
    /// class file.
    pub fn load(&self, name: &ClassName) -> Result<Class, ClassLoadError> {
        match self.load_class(name) {
            Ok(class) => Ok(class),
            Err(e) => {
                Err(ClassLoadError::new(name.clone(), e))
            }
        }
    }

    fn load_class(&self, name: &ClassName) -> Result<Class, ClassLoadErrorKind> {
        let mut class_path: PathBuf = name.as_ref().split('.').collect();
        class_path.set_extension("mvm");

        for path in self.paths.iter().cloned() {
            let path: PathBuf = path;
            let path = path.join(&class_path);

            if let Ok(mut file) = File::open(path) {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let class_info = ClassFileParser::new(&contents).parse()?;
                let class: Class = class_info.try_into()?;

                if class.name() != name {
                    return Err(ClassLoadErrorKind::WrongName {
                        name: class.name().clone()
                    });
                }

                return Ok(class);
            }
        }

        Err(ClassLoadErrorKind::ClassNotFound)
    }
}