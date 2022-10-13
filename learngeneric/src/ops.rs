use std::ops::{Add, Div, Mul, Sub};

// 运算符重载
fn multiply<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[derive(Debug)]
pub struct Score {
    first: f32,
    second: f32,
}

impl Score {
    pub fn new(first: f32, second: f32) -> Self {
        Self { first, second }
    }
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
        }
    }
}

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "miemie".to_owned()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "momomo".to_owned()
    }
}

/// trait 不能直接作为参数返回
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

/// trait 作为参数

fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add + Add<Output = T>,
{
    a + b
}

#[cfg(test)]
mod test {
    use crate::ops::{multiply, sum, Score};

    #[test]
    fn test_mul() {
        println!("mul {}", multiply(1024, 10));
        println!("mul {}", multiply(2.2, 3.3));

        let s1 = Score::new(1.2, 2.3);
        let s2 = Score::new(2.3, 1.2);

        println!("add response :{:?}", s1 + s2);

        println!("sum  {}", sum(1024, 1024));
    }
}
