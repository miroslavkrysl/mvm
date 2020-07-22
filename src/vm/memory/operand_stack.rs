use std::convert::{TryFrom, TryInto};
use std::sync::Mutex;

use crate::vm::memory::error::OperandStackError;
use crate::vm::types::value::{Value, ValueCategory};


/// An operand stack.
#[derive(Debug)]
pub struct OperandStack {
    values: Mutex<Vec<Value>>,
    size: Mutex<usize>,
    capacity: Mutex<usize>,
}


impl OperandStack {
    /// Creates a new OperandStack
    pub fn new(capacity: usize) -> Self {
        OperandStack {
            values: Mutex::new(Vec::with_capacity(capacity)),
            size: Mutex::new(0),
            capacity: Mutex::new(capacity),
        }
    }

    fn check_overflow(&self, size: usize) -> Result<(), OperandStackError> {
        if *self.size.lock().unwrap() + size > *self.capacity.lock().unwrap() {
            Err(OperandStackError::Overflow)
        } else {
            Ok(())
        }
    }

    fn check_underflow(&self, size: usize) -> Result<(), OperandStackError> {
        if size > *self.size.lock().unwrap() {
            Err(OperandStackError::Underflow)
        } else {
            Ok(())
        }
    }

    /// Duplicate top dup values (total comp size = dup) and
    /// insert it on to the stack skipping the dup + skip values.
    fn dup_skip(&self, dup: usize, skip: usize) -> Result<(), OperandStackError> {
        if dup == 0 {
            return Ok(());
        }

        self.check_underflow(dup + skip)?;
        self.check_overflow(dup)?;

        let mut values = self.values.lock().unwrap();

        // compute number of values to duplicate
        let mut values_size = 0;
        let mut dup_count = 0;
        loop {
            if values_size == dup {
                break;
            }

            if values_size > dup {
                return Err(OperandStackError::InvalidType);
            }

            dup_count += 1;
            values_size += values[values.len() - dup_count].value_type().category().size();
        };

        // compute number of values to skip
        let mut values_size = 0;
        let mut skip_count = 0;
        loop {
            if values_size == skip {
                break;
            }

            if values_size > skip {
                return Err(OperandStackError::InvalidType);
            }

            skip_count += 1;
            values_size += values[values.len() - skip_count].value_type().category().size();
        };

        // remove skip + dup values
        let len = values.len();
        let to_dup: Vec<_> = values.drain((len - dup_count - skip_count)..).collect();

        // append dup values
        values.extend_from_slice(&to_dup[(to_dup.len() - dup_count)..]);
        // append skip + dup values to the end
        values.extend(to_dup);

        *self.size.lock().unwrap() += dup;

        Ok(())
    }

    /// Pop and discard top values of the given total comp size.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    pub fn pop_discard(&self, size: usize) -> Result<(), OperandStackError> {
        self.check_underflow(size)?;
        let mut values = self.values.lock().unwrap();

        // compute new length of values after pop
        let mut values_size = 0;
        let mut new_len = values.len();
        loop {
            if values_size == size {
                break;
            }

            if values_size > size {
                return Err(OperandStackError::InvalidType);
            }

            new_len -= 1;
            values_size += values[new_len].value_type().category().size();
        };

        values.truncate(new_len);
        *self.size.lock().unwrap() -= size;

        Ok(())
    }

    /// Peek a generic value from top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Underflow`
    /// if the stack is empty.
    pub fn peek_value(&self, index: usize) -> Result<Value, OperandStackError> {
        let values = self.values.lock().unwrap();
        if index >= values.len() {
            return Err(OperandStackError::Underflow);
        }

        Ok(values[values.len() - index - 1].clone())
    }

    /// Peek a value from top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Underflow`
    /// if the stack is empty.
    pub fn peek<T>(&self, index: usize) -> Result<T, OperandStackError>
                   where T: TryFrom<Value>,
                         OperandStackError: From<<T as TryFrom<Value>>::Error> {
        let values = self.values.lock().unwrap();
        if index >= values.len() {
            return Err(OperandStackError::Underflow);
        }

        Ok(values[values.len() - index - 1].clone().try_into()?)
    }

    /// Duplicate the top value of the comp size 1.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough value on the stack.
    pub fn dup1(&self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 0)
    }

    /// Duplicate the top value of the comp size 1
    /// and insert it onto the stack skipping the
    /// 2 values of the comp size 1.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough values on the stack or
    /// `OperandStackError::Overflow` of there is no more
    /// room on the stack.
    pub fn dup1_skip1(&self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 1)
    }

    /// Duplicate the top value of the comp size 1
    /// and insert it onto the stack skipping the
    /// values of the total comp size 3.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough values on the stack or
    /// `OperandStackError::Overflow` of there is no more
    /// room on the stack.
    pub fn dup1_skip2(&self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 2)
    }

    /// Duplicate the top values of the total comp size 2.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough values on the stack or
    /// `OperandStackError::Overflow` of there is no more
    /// room on the stack.
    pub fn dup2(&self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 0)
    }

    /// Duplicate the top values of the total comp size 2
    /// and insert it onto the stack skipping the
    /// values of the total comp size 3.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough values on the stack or
    /// `OperandStackError::Overflow` of there is no more
    /// room on the stack.
    pub fn dup2_skip1(&self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 1)
    }

    /// Duplicate the top values of the total comp size 2
    /// and insert it onto the stack skipping the
    /// values of the total comp size 4.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough values on the stack or
    /// `OperandStackError::Overflow` of there is no more
    /// room on the stack.
    pub fn dup2_skip2(&self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 2)
    }

    /// Pop and discard top values of the total comp size = 1.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough value on the stack.
    pub fn pop_discard1(&self) -> Result<(), OperandStackError> {
        self.pop_discard(1)
    }


    /// Pop and discard top values of the total comp size = 2.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough value on the stack.
    pub fn pop_discard2(&self) -> Result<(), OperandStackError> {
        self.pop_discard(2)
    }

    /// Swap the top twe values of the comp size 1.
    ///
    /// # Errors
    ///
    /// Returns an `OperandStackError::InvalidType` if the operation
    /// affect only a half of any value of comp type 2.
    /// Returns an `OperandStackError::Underflow` if there
    /// are not enough value on the stack.
    pub fn swap(&self) -> Result<(), OperandStackError> {
        let value0 = self.peek_value(0)?;
        let value1 = self.peek_value(1)?;

        let mut values = self.values.lock().unwrap();

        if value0.value_type().category() != ValueCategory::Single
            || value1.value_type().category() != ValueCategory::Single {
            return Err(OperandStackError::InvalidType);
        }

        let len = values.len();
        values.swap(len - 1, len - 2);

        Ok(())
    }

    /// Pop a value from top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Underflow`
    /// if the stack is empty.
    pub fn pop<T>(&self) -> Result<T, OperandStackError>
                  where T: TryFrom<Value>,
                        OperandStackError: From<<T as TryFrom<Value>>::Error> {
        let mut values = self.values.lock().unwrap();
        match values.last() {
            None => Err(OperandStackError::Underflow),
            Some(comp_value) => {
                let value = T::try_from(comp_value.clone())?;
                values.pop();
                Ok(value)
            }
        }
    }

    /// Pop a generic value from top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Underflow`
    /// if the stack is empty.
    pub fn pop_value(&self) -> Result<Value, OperandStackError> {
        let mut values = self.values.lock().unwrap();
        match values.pop() {
            None => Err(OperandStackError::Underflow),
            Some(value) => {
                *self.size.lock().unwrap() -= value.value_type().category().size();
                Ok(value)
            }
        }
    }

    /// Push a value on top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Overflow` if the
    /// maximum stack capacity is reached.
    pub fn push<T: Into<Value>>(&self, value: T) -> Result<(), OperandStackError> {
        self.push_value(value.into())
    }

    /// Push a generic value on top of the stack.
    ///
    /// # Errors
    ///
    /// Can return `OperandStack::Overflow` if the
    /// maximum stack capacity is reached.
    pub fn push_value(&self, value: Value) -> Result<(), OperandStackError> {
        self.check_overflow(value.value_type().category().size())?;

        *self.size.lock().unwrap() += value.value_type().category().size();
        self.values.lock().unwrap().push(value);

        Ok(())
    }

    /// Returns all the values of the local variables array.
    pub fn values(&self) -> Vec<Value> {
        self.values.lock().unwrap().clone()
    }
}


#[cfg(test)]
mod test {
    use crate::vm::memory::operand_stack::OperandStack;
    use crate::vm::types::double::Double;
    use crate::vm::types::float::Float;
    use crate::vm::types::int::Int;
    use crate::vm::types::long::Long;
    use crate::vm::types::reference::Reference;


    #[test]
    fn push_and_pop() {
        let stack = OperandStack::new(32);

        stack.push(Int::new(1)).unwrap();
        stack.push(Long::new(2)).unwrap();
        stack.push(Float::new(3.0)).unwrap();
        stack.push(Double::new(4.0)).unwrap();
        stack.push(Reference::null()).unwrap();

        assert_eq!(stack.pop::<Reference>().unwrap(), Reference::null());
        assert_eq!(stack.pop::<Double>().unwrap(), Double::new(4.0));
        assert_eq!(stack.pop::<Float>().unwrap(), Float::new(3.0));
        assert_eq!(stack.pop::<Long>().unwrap(), Long::new(2));
        assert_eq!(stack.pop::<Int>().unwrap(), Int::new(1));
    }


    #[test]
    fn overflow() {
        let stack = OperandStack::new(6);

        stack.push(Int::new(1)).unwrap();
        stack.push(Long::new(2)).unwrap();
        stack.push(Float::new(3.0)).unwrap();
        stack.push(Double::new(4.0)).unwrap();
        stack.push(Reference::null()).expect_err("push should return overflow error");
    }


    #[test]
    fn underflow() {
        let stack = OperandStack::new(32);
        stack.pop_value().expect_err("pop on empty stack should return underflow error");

        stack.push(Int::new(1)).unwrap();
        stack.push(Long::new(2)).unwrap();
        stack.push(Float::new(3.0)).unwrap();
        stack.push(Double::new(4.0)).unwrap();
        stack.push(Reference::null()).unwrap();

        stack.pop_value().unwrap();
        stack.pop_value().unwrap();
        stack.pop_value().unwrap();
        stack.pop_value().unwrap();
        stack.pop_value().unwrap();
        stack.pop_value().expect_err("pop should return underflow error");
    }


    #[test]
    fn dup1() {
        let mut stack = OperandStack::new(32);
        stack.dup1().expect_err("dup1 should return underflow error");

        stack.push(Long::new(2)).unwrap();
        stack.dup1().expect_err("dup1 should return invalid type error");
        stack.push(Double::new(4.0)).unwrap();
        stack.dup1().expect_err("dup1 should return invalid type error");
        stack.push(Int::new(1)).unwrap();
        stack.dup1().expect("dup1 on int should be ok");
        stack.push(Float::new(3.0)).unwrap();
        stack.dup1().expect("dup1 on float should be ok");
        stack.push(Reference::null()).unwrap();
        stack.dup1().expect("dup1 on reference should be ok");

        assert_eq!(stack.pop::<Reference>().unwrap(), Reference::null());
        assert_eq!(stack.pop::<Reference>().unwrap(), Reference::null());
        assert_eq!(stack.pop::<Float>().unwrap(), Float::new(3.0));
        assert_eq!(stack.pop::<Float>().unwrap(), Float::new(3.0));
        assert_eq!(stack.pop::<Int>().unwrap(), Int::new(1));
        assert_eq!(stack.pop::<Int>().unwrap(), Int::new(1));
        assert_eq!(stack.pop::<Double>().unwrap(), Double::new(4.0));
        assert_eq!(stack.pop::<Long>().unwrap(), Long::new(2));
        stack.pop_value().expect_err("stack should be empty but is not");
    }


    #[test]
    fn pop() {}


    #[test]
    fn swap() {}
}