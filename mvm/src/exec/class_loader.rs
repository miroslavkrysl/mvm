use crate::class::name::ClassName;
use crate::class::class::Class;
use crate::exec::error::ClassLoadError;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use crate::parse::parser::ClassFileParser;
use std::convert::TryInto;


pub struct ClassLoader {
    paths: Vec<PathBuf>
}

impl ClassLoader {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        ClassLoader {
            paths
        }
    }

    pub fn load_class(&self, name: &ClassName) -> Result<Class, ClassLoadError> {
        let mut class_path = PathBuf::from(name.as_ref());
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
                    return Err(ClassLoadError::WrongName {
                        expected: name.clone(),
                        got: class.name().clone()
                    });
                }

                return Ok(class);
            }
        }

        Err(ClassLoadError::ClassNotFound(name.clone()))
    }
}