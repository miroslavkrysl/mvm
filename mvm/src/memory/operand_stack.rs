use std::convert::TryFrom;

use crate::memory::OperandStackError;
use crate::types::{Categorize, CompValue, ValueCategory};

#[derive(Debug, Clone)]
pub struct OperandStack {
    values: Vec<CompValue>,
    size: usize,
    capacity: usize,
}

impl OperandStack {
    pub fn new(capacity: usize) -> Self {
        OperandStack {
            values: Vec::with_capacity(capacity),
            size: 0,
            capacity,
        }
    }

    pub fn check_overflow(&self, size: usize) -> Result<(), OperandStackError> {
        if self.size + size > self.capacity {
            Err(OperandStackError::Overflow)
        } else {
            Ok(())
        }
    }

    pub fn check_underflow(&self, size: usize) -> Result<(), OperandStackError> {
        if size > self.size {
            Err(OperandStackError::Underflow)
        } else {
            Ok(())
        }
    }

    pub fn dup_skip(&mut self, dup: usize, skip: usize) -> Result<(), OperandStackError> {
        if dup == 0 {
            return Ok(());
        }

        self.check_underflow(dup + skip)?;
        self.check_overflow(dup)?;

        // compute index of values to duplicate
        let mut values_size = 0;
        let mut dup_index = self.values.len();
        loop {
            if values_size == dup {
                break;
            }

            if values_size > dup {
                return Err(OperandStackError::InvalidType);
            }

            dup_index -= 1;
            values_size += self.values[dup_index].category().size();
        };

        // compute index of values to skip
        let mut values_size = 0;
        let mut skip_index = dup_index;
        loop {
            if values_size == skip {
                break;
            }

            if values_size > skip {
                return Err(OperandStackError::InvalidType);
            }

            skip_index -= 1;
            values_size += self.values[skip_index].category().size();
        };

        let len = self.values.len();
        let new_len = len + len - dup_index;

        // append dup values to the end
        unsafe {
            self.values.set_len(new_len);
            self.values.copy_within(dup_index..len, len);
        }

        self.size += dup;

        // copy skip values right before the duplicated values
        self.values.copy_within(skip_index..dup_index, len - (dup_index - skip_index));

        // copy dup values right before the copy of skip values
        self.values.copy_within(len.., skip_index);

        Ok(())
    }

    pub fn pop_discard(&mut self, size: usize) -> Result<(), OperandStackError> {
        self.check_underflow(size);

        // compute new length of values after pop
        let mut values_size = 0;
        let mut new_len = self.values.len();
        loop {
            if values_size == size {
                break;
            }

            if values_size > size {
                return Err(OperandStackError::InvalidType);
            }

            new_len -= 1;
            values_size += self.values[new_len].category().size();
        };

        self.values.truncate(new_len);
        self.size -= size;

        Ok(())
    }

    pub fn peek(&self, index: usize) -> Result<CompValue, OperandStackError> {
        if index >= self.values.len() {
            return Err(OperandStackError::Underflow);
        }

        Ok(self.values[self.values.len() - index - 1])
    }

    pub fn dup1(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 0)
    }

    pub fn dup1_skip1(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 1)
    }

    pub fn dup1_skip2(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(1, 2)
    }

    pub fn dup2(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 0)
    }

    pub fn dup2_skip1(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 1)
    }

    pub fn dup2_skip2(&mut self) -> Result<(), OperandStackError> {
        self.dup_skip(2, 2)
    }

    pub fn pop_discard1(&mut self) -> Result<(), OperandStackError> {
        self.pop_discard(1)
    }

    pub fn pop_discard2(&mut self) -> Result<(), OperandStackError> {
        self.pop_discard(2)
    }

    pub fn swap(&mut self) -> Result<(), OperandStackError> {
        let value0 = self.peek(0)?;
        let value1 = self.peek(1)?;

        if value0.category() != ValueCategory::Single || value1.category() != ValueCategory::Single {
            return Err(OperandStackError::InvalidType);
        }

        let len = self.values.len();
        self.values.swap(len - 1, len - 2);
        Ok(())
    }

    pub fn pop<T>(&mut self) -> Result<T, OperandStackError>
        where T: TryFrom<CompValue>,
              OperandStackError: From<<T as TryFrom<CompValue>>::Error> {
        match self.values.last() {
            None => Err(OperandStackError::Underflow),
            Some(comp_value) => {
                let value = T::try_from(comp_value.clone())?;
                self.values.pop();
                Ok(value)
            },
        }
    }

    pub fn pop_value(&mut self) -> Result<CompValue, OperandStackError> {
        match self.values.pop() {
            None => Err(OperandStackError::Underflow),
            Some(value) => {
                self.size -= value.category().size();
                Ok(value)
            },
        }
    }

    pub fn push<T: Into<CompValue>>(&mut self, value: T) -> Result<(), OperandStackError> {
        self.push_value(value.into())
    }

    pub fn push_value(&mut self, value: CompValue) -> Result<(), OperandStackError> {
        self.check_overflow(value.category().size())?;

        self.values.push(value);
        self.size += value.category().size();

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::memory::operand_stack::OperandStack;
    use crate::types::{Int, Long, Float, Double, Reference};

    #[test]
    fn push_and_pop() {
        let mut stack = OperandStack::new(32);

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
        let mut stack = OperandStack::new(6);

        stack.push(Int::new(1)).unwrap();
        stack.push(Long::new(2)).unwrap();
        stack.push(Float::new(3.0)).unwrap();
        stack.push(Double::new(4.0)).unwrap();
        stack.push(Reference::null()).expect_err("push should return overflow error");
    }

    #[test]
    fn underflow() {
        let mut stack = OperandStack::new(32);
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
    fn pop() {

    }

    #[test]
    fn swap() {

    }
}