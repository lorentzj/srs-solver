use std::ops;
use serde::Serialize;

// overflow-safe 127 bit fixed point rational type
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Rat {
    num: i64,
    den: i64
}

impl Rat {
    pub fn new(val: i64) -> Rat {
        Rat {
            num: val,
            den: 1
        }
    }

    pub fn try_int(&self) -> Option<i64> {
        if self.den == 1 {
            Some(self.num)
        } else {
            None
        }
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
}

impl ops::Add<Rat> for Rat {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self {
        loop {
            let den_gcd = gcd(self.den, rhs.den);

            let lhs_num = match (rhs.den / den_gcd).checked_mul(self.num) {
                Some(v) => v,
                None => {
                    self.num >>= 1;
                    self.den >>= 1;

                    continue;    
                }
            };
    
            let rhs_num = match (self.den / den_gcd).checked_mul(rhs.num) {
                Some(v) => v,
                None => {
                    rhs.num >>= 1;
                    rhs.den >>= 1;

                    continue;
                }
            };
    
            let num = match lhs_num.checked_add(rhs_num) {
                Some(v) => v,
                None => {
                    if self.num > rhs.num {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1;
                    }

                    continue;
                }
            };
    
            let den = match (self.den / den_gcd).checked_mul(rhs.den) {
                Some(v) => v,
                None => {
                    if self.den > rhs.den {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1; 
                    }    

                    continue;
                }
            };

            let new_gcd = gcd(num, den).abs();
    
            return Self { num: num / new_gcd, den: den / new_gcd }    
        }
    }
}

impl ops::Sub<Rat> for Rat {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self {
        loop {
            let den_gcd = gcd(self.den, rhs.den);

            let lhs_num = match (rhs.den / den_gcd).checked_mul(self.num) {
                Some(v) => v,
                None => {
                    self.num >>= 1;
                    self.den >>= 1;

                    continue;    
                }
            };
    
            let rhs_num = match (self.den / den_gcd).checked_mul(rhs.num) {
                Some(v) => v,
                None => {
                    rhs.num >>= 1;
                    rhs.den >>= 1;

                    continue;
                }
            };
    
            let num = match lhs_num.checked_sub(rhs_num) {
                Some(v) => v,
                None => {
                    if self.num > rhs.num {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1;
                    }

                    continue;
                }
            };
    
            let den = match (self.den / den_gcd).checked_mul(rhs.den) {
                Some(v) => v,
                None => {
                    if self.den > rhs.den {
                        self.num >>= 1;
                        self.den >>= 1;    
                    } else {
                        rhs.num >>= 1;
                        rhs.den >>= 1; 
                    }    

                    continue;
                }
            };

            let new_gcd = gcd(num, den).abs();
    
            return Self { num: num / new_gcd, den: den / new_gcd }    
        }
    }
}

impl ops::Mul<Rat> for Rat {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self {
        loop {
            let lhs_gcd = gcd(self.num, rhs.den);
            let rhs_gcd = gcd(rhs.num, self.den);
            
            let num = (self.num / lhs_gcd).checked_mul(rhs.num / rhs_gcd);
            let den = (self.den / rhs_gcd).checked_mul(rhs.den / lhs_gcd);
    
            if let (Some(num), Some(den)) = (num, den) {
                return Self { num, den }
            } else if self.num > rhs.num {
                self.num >>= 1;
                self.den >>= 1;
            } else {
                rhs.num >>= 1;
                rhs.den >>= 1;
            }    
        }
    }
}

// Euclid's algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut t: i64;

    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::Rat;

    #[test]
    fn arith() {
        let a = Rat::new(1);
        let b = Rat::new(2);
        assert_eq!((a + b).num, 3);
        assert_eq!((a + b).den, 1);

        assert_eq!((a - b).num, -1);
        assert_eq!((a + b).den, 1);

        assert_eq!(((a + b) * (a - b) + b).num, -1);
    }
}