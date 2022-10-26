use std::fmt::Display;

use rand::Rng;

/// The binary representation of a number in the genetic algorithm
pub struct Bits {
    bits: [bool; 3],
}

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .bits
            .iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect::<Vec<&str>>()
            .join("");
        write!(f, "{}", s)
    }
}

/// Represents a number in the range [0, 7].
///
/// The number is represented by the least significant 3 bits of a u8.
pub struct Number {
    value: u8,
}

impl Number {
    /// Creates a new number from a u8, if the number is in the range [0, 7].
    // pub fn new(value: u8) -> Result<Number, &'static str> {
    //     if value < 8 {
    //         Ok(Number { value })
    //     } else {
    //         Err("Number must be between 0 and 7")
    //     }
    // }

    /// Returns the integer value of the number
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Returns a conversion of the number to f64
    pub fn f64(&self) -> f64 {
        f64::from(self.value)
    }

    /// Generates a random number in the range [0, 7]
    pub fn random() -> Number {
        let value = rand::thread_rng().gen_range(0..8);
        Number { value }
    }

    /// Returns the binary representation of the number
    pub fn bits(&self) -> Bits {
        let bits = [
            self.value & 0b100 != 0,
            self.value & 0b010 != 0,
            self.value & 0b001 != 0,
        ];
        Bits { bits }
    }
}

/// Represents a pair of numbers in the range [0, 7], used to represent a point in the cartesian plane.
///
/// The pair is represented by the least significant 6 bits of a u8.
///
/// The first 3 bits represent the x coordinate and the last 3 bits represent the y coordinate.
#[derive(Clone, Copy)]
pub struct Pair {
    value: u8,
}

impl Pair {
    /// Creates a new pair from two numbers
    pub fn new(x: Number, y: Number) -> Pair {
        Pair {
            value: (x.value() << 3) | y.value(),
        }
    }

    /// Returns the x coordinate of the pair
    pub fn x(&self) -> Number {
        Number {
            value: self.value >> 3,
        }
    }

    /// Returns the y coordinate of the pair
    pub fn y(&self) -> Number {
        Number {
            value: self.value & 0b00000111,
        }
    }

    /// Returns a random pair of numbers in the range [0, 7]
    pub fn random() -> Pair {
        Pair::new(Number::random(), Number::random())
    }

    /// Performs crossover between two pairs
    pub fn crossover(&self, other: &Pair) -> Vec<Pair> {
        let mut pairs = Vec::new();

        for crossover_point in 1..=5 {
            let mask = 0b00111111 >> crossover_point;

            let c1p1 = self.value & mask;
            let c1p2 = other.value & !mask;
            let c1 = c1p1 | c1p2;

            let c2p1 = other.value & mask;
            let c2p2 = self.value & !mask;
            let c2 = c2p1 | c2p2;

            pairs.push(Pair { value: c1 });
            pairs.push(Pair { value: c2 });
        }

        pairs
    }

    /// Performs mutation on a pair by flipping a random bit
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let bit = rng.gen_range(0..6);
        self.value ^= 0b00100000 >> bit;
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x().value(), self.y().value())
    }
}
