use std::ops::{Add, Sub, Mul, Div, Rem};


/// Used for JVM type-to-type conversion.
/// Is possible for the conversion to be lossy.
pub trait Convert<T> {
    /// Performs the conversion.
    fn convert(&self) -> T;
}


/// A basic operand unit with the width of one word.
/// Internally represented as `u32` Rust integer.
#[derive(Debug, Copy, Clone)]
pub struct Word(u32);

impl Word {
    pub fn new(value: u32) -> Self {
        Word(value)
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Word(value)
    }
}

impl From<Word> for u32 {
    fn from(word: Word) -> Self {
        word.0
    }
}


/// A basic operand unit with the width of two words.
/// Internally represented as `u64` Rust integer.
#[derive(Debug, Copy, Clone)]
pub struct DoubleWord(u64);

impl DoubleWord {
    pub fn new(value: u64) -> Self {
        DoubleWord(value)
    }
}

impl From<u64> for DoubleWord {
    fn from(value: u64) -> Self {
        DoubleWord(value)
    }
}

impl From<DoubleWord> for u64 {
    fn from(double_word: DoubleWord) -> Self {
        double_word.0
    }
}


/// A JVM `int` computational type.
/// Internally represented using the `i32` Rust integer type.
#[derive(Debug, Copy, Clone)]
pub struct Int(i32);

impl Int {
    pub fn new(value: i32) -> Self {
        Int(value)
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Int(value)
    }
}

impl From<Int> for i32 {
    fn from(int: Int) -> Self {
        int.0
    }
}

impl From<Word> for Int {
    fn from(word: Word) -> Self {
        Int(word.0 as i32)
    }
}

impl From<Int> for Word {
    fn from(int: Int) -> Self {
        Word(int.0 as u32)
    }
}

impl Add<Int> for Int {
    type Output = Int;

    fn add(self, rhs: Int) -> Self::Output {
        Int(self.0.wrapping_add(rhs.0))
    }
}

impl Sub<Int> for Int {
    type Output = Int;

    fn sub(self, rhs: Int) -> Self::Output {
        Int(self.0.wrapping_sub(rhs.0))
    }
}

impl Mul<Int> for Int {
    type Output = Int;

    fn mul(self, rhs: Int) -> Self::Output {
        Int(self.0.wrapping_mul(rhs.0))
    }
}

impl Convert<Long> for Int {
    fn convert(&self) -> Long {
        Long(self.0 as i64)
    }
}

impl Convert<Float> for Int {
    fn convert(&self) -> Float {
        Float(self.0 as f32)
    }
}

impl Convert<Double> for Int {
    fn convert(&self) -> Double {
        Double(self.0 as f64)
    }
}

impl Convert<Byte> for Int {
    fn convert(&self) -> Byte {
        Byte(self.0 as i8)
    }
}

impl Convert<Short> for Int {
    fn convert(&self) -> Short {
        Short(self.0 as i16)
    }
}

impl Convert<Boolean> for Int {
    fn convert(&self) -> Boolean {
        Boolean((self.0 & 1) as u8)
    }
}

impl Convert<Char> for Int {
    fn convert(&self) -> Char {
        Char(self.0 as u16)
    }
}


/// A JVM `long` computational type.
/// Internally represented using the `i64` Rust integer type.
#[derive(Debug, Copy, Clone)]
pub struct Long(i64);

impl Long {
    pub fn new(value: i64) -> Self {
        Long(value)
    }
}

impl From<i64> for Long {
    fn from(value: i64) -> Self {
        Long(value)
    }
}

impl From<Long> for i64 {
    fn from(long: Long) -> Self {
        long.0
    }
}

impl From<DoubleWord> for Long {
    fn from(word: DoubleWord) -> Self {
        Long(word.0 as i64)
    }
}

impl From<Long> for DoubleWord {
    fn from(long: Long) -> Self {
        DoubleWord(long.0 as u64)
    }
}

impl Add<Long> for Long {
    type Output = Long;

    fn add(self, rhs: Long) -> Self::Output {
        Long(self.0.wrapping_add(rhs.0))
    }
}

impl Sub<Long> for Long {
    type Output = Long;

    fn sub(self, rhs: Long) -> Self::Output {
        Long(self.0.wrapping_sub(rhs.0))
    }
}

impl Mul<Long> for Long {
    type Output = Long;

    fn mul(self, rhs: Long) -> Self::Output {
        Long(self.0.wrapping_mul(rhs.0))
    }
}

impl Convert<Int> for Long {
    fn convert(&self) -> Int {
        Int(self.0 as i32)
    }
}

impl Convert<Float> for Long {
    fn convert(&self) -> Float {
        Float(self.0 as f32)
    }
}

impl Convert<Double> for Long {
    fn convert(&self) -> Double {
        Double(self.0 as f64)
    }
}

/// A JVM `float` computational type.
/// Internally represented using the `f32` Rust float type.
#[derive(Debug, Copy, Clone)]
pub struct Float(f32);

impl Float {
    pub fn new(value: f32) -> Self {
        Float(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value)
    }
}

impl From<Float> for f32 {
    fn from(float: Float) -> Self {
        float.0
    }
}

impl From<Word> for Float {
    fn from(word: Word) -> Self {
        Float(f32::from_bits(word.0))
    }
}

impl From<Float> for Word {
    fn from(float: Float) -> Self {
        Word(float.0.to_bits())
    }
}

impl Add<Float> for Float {
    type Output = Float;

    fn add(self, rhs: Float) -> Self::Output {
        Float(self.0 + rhs.0)
    }
}

impl Sub<Float> for Float {
    type Output = Float;

    fn sub(self, rhs: Float) -> Self::Output {
        Float(self.0 - rhs.0)
    }
}

impl Mul<Float> for Float {
    type Output = Float;

    fn mul(self, rhs: Float) -> Self::Output {
        Float(self.0 * rhs.0)
    }
}

impl Convert<Int> for Float {
    fn convert(&self) -> Int {
        Int(self.0 as i32)
    }
}

impl Convert<Long> for Float {
    fn convert(&self) -> Long {
        Long(self.0 as i64)
    }
}

impl Convert<Double> for Float {
    fn convert(&self) -> Double {
        Double(self.0 as f64)
    }
}

/// A JVM `double` computational type.
/// Internally represented using the `f64` Rust float type.
#[derive(Debug, Copy, Clone)]
pub struct Double(f64);

impl Double {
    pub fn new(value: f64) -> Self {
        Double(value)
    }
}

impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double(value)
    }
}

impl From<Double> for f64 {
    fn from(double: Double) -> Self {
        double.0
    }
}

impl From<DoubleWord> for Double {
    fn from(word: DoubleWord) -> Self {
        Double(f64::from_bits(word.0))
    }
}

impl From<Double> for DoubleWord {
    fn from(double: Double) -> Self {
        DoubleWord(double.0.to_bits())
    }
}

impl Add<Double> for Double {
    type Output = Double;

    fn add(self, rhs: Double) -> Self::Output {
        Double(self.0 + rhs.0)
    }
}

impl Sub<Double> for Double {
    type Output = Double;

    fn sub(self, rhs: Double) -> Self::Output {
        Double(self.0 - rhs.0)
    }
}

impl Mul<Double> for Double {
    type Output = Double;

    fn mul(self, rhs: Double) -> Self::Output {
        Double(self.0 * rhs.0)
    }
}

impl Convert<Int> for Double {
    fn convert(&self) -> Int {
        Int(self.0 as i32)
    }
}

impl Convert<Long> for Double {
    fn convert(&self) -> Long {
        Long(self.0 as i64)
    }
}

impl Convert<Float> for Double {
    fn convert(&self) -> Float {
        Float(self.0 as f32)
    }
}


/// A JVM `byte` type.
/// Internally represented using the `i8` Rust integer type.
pub struct Byte(i8);


/// A JVM `short` type.
/// Internally represented using the `i16` Rust integer type.
pub struct Short(i16);


/// A JVM `boolean` type.
/// Internally represented using the `u8` Rust integer type.
pub struct Boolean(u8);


/// A JVM `char` type.
/// Internally represented using the `u16` Rust integer type.
pub struct Char(u16);
