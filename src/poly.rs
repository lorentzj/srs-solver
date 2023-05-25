use serde::{Serialize, Serializer};
use std::fmt;
use std::{collections::VecDeque, rc::Rc};

use crate::mono::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Poly {
    pub terms: VecDeque<Mono>,
    pub var_dict: Rc<Vec<String>>,
}

impl Poly {
    pub fn var(var: usize, pow: u64, var_dict: &Rc<Vec<String>>) -> Self {
        if pow == 0 {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![],
                }]),
                var_dict: var_dict.clone(),
            }
        } else {
            Self {
                terms: VecDeque::from(vec![Mono {
                    num: 1,
                    den: 1,
                    vars: vec![(var, pow)],
                }]),
                var_dict: var_dict.clone(),
            }
        }
    }

    pub fn constant(val: i64, var_dict: &Rc<Vec<String>>) -> Self {
        Self {
            terms: if val == 0 {
                VecDeque::new()
            } else {
                VecDeque::from(vec![Mono {
                    num: val,
                    den: 1,
                    vars: vec![],
                }])
            },
            var_dict: var_dict.clone(),
        }
    }

    pub fn get_constant_val(&self) -> Option<(i64, i64)> {
        if self.terms.is_empty() {
            Some((0, 1))
        } else if self.terms.len() == 1 {
            if self.terms[0].vars.is_empty() {
                Some((self.terms[0].num, self.terms[0].den))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl fmt::Debug for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            write!(f, "0")?
        }

        for (i, Mono { num, den, vars }) in (self.terms).iter().enumerate() {
            let coef = (*num as f64)/(*den as f64);
            if coef != 1. || vars.is_empty() {
                if coef < 0. {
                    if coef == -1. && !vars.is_empty() {
                        if i == 0 {
                            write!(f, "-")?;
                        } else {
                            write!(f, " - ")?;
                        }
                    } else if i == 0 {
                        write!(f, "{coef}")?;
                    } else {
                        write!(f, " - {}", -coef)?;
                    }
                } else if i == 0 {
                    write!(f, "{coef}")?;
                } else {
                    write!(f, " + {coef}")?;
                }
            } else if i != 0 {
                write!(f, " + ")?;
            }

            for (var, pow) in vars {
                if *pow == 1 {
                    write!(f, "{}", self.var_dict[*var])?;
                } else {
                    write!(f, "{}^{pow}", self.var_dict[*var])?;
                }
            }
        }

        Ok(())
    }
}

impl Serialize for Poly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("{self:?}"))
    }
}
