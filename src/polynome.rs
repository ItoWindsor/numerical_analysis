#![allow(non_camel_case_types)]

use std::collections::HashMap;       

//use crate::common_type::CommonType_t;

pub struct Polynome_t<T : num::Num + Default + Clone + Copy> {
    deg_to_coeff : HashMap<u32, T>,
}

impl<T : num::Num + Default + Clone + Copy> Polynome_t<T> {
    pub fn new(deg_coeff : &HashMap<u32,T>) -> Polynome_t<T> {
        return Self {deg_to_coeff : deg_coeff.clone()};
    }
    
    // TODO : Implement a method to get the polynome from two arrays : deg and coeff 
    
    pub fn from_array<const M : usize>(deg_arr : [u32;M], coeff_arr : [T;M]) -> Self {
        let mut deg_to_coeff = HashMap::new();
        for i in 0..M {
            deg_to_coeff.insert(deg_arr[i], coeff_arr[i]);
        }
        return Polynome_t { deg_to_coeff };
    }
    
}

// TODO : Add a method to sum two different polynomes  

//impl<T,U,V> std::ops::Add<&Polynome_t<U>> for &Polynome_t<T>
//    where 
//    T: num::Num + Default + Clone + Copy + num::NumCast,
//    U: num::Num + Default + Clone + Copy + num::NumCast,
//    () : CommonType_t<T,U, Output = V>,
//    V: num::Num + Default + Clone + Copy + num::NumCast {
//    type Output = Polynome_t<V>;
//
//    fn add(self, poly2 : &Polynome_t<U>) -> Polynome_t<V> {
//        
//    }
//}

impl <T : num::Num + Default + Clone + std::fmt::Display + Copy + std::cmp::PartialOrd + std::ops::Neg> std::fmt::Display for Polynome_t<T>
    where <T  as std::ops::Neg>::Output: std::fmt::Display{
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result{ 
        let mut res : String = String::new();
        let mut keys : Vec<u32> = self.deg_to_coeff.keys().cloned().collect();
        keys.sort();
        keys.reverse();

        for deg in keys {
            let coeff = self.deg_to_coeff[&deg];
            if res.is_empty() {
                if deg == 0 {
                    res.push_str(&format!("{}", coeff));
                } else if deg == 1 {
                    res.push_str(&format!("{}X", coeff));
                } else {
                    res.push_str(&format!("{}X^{}", coeff, deg));
                }
            } 
            else {
                if coeff < T::zero() {
                    if deg == 0 {
                        res.push_str(&format!(" - {}", -coeff));
                    } else if deg == 1 {
                        res.push_str(&format!(" - {}X", -coeff));
                    } else {
                        res.push_str(&format!(" - {}X^{}", -coeff, deg));
                    }
                } else {
                    if deg == 0 {
                        res.push_str(&format!(" + {}", coeff));
                    } else if deg == 1 {
                        res.push_str(&format!(" + {}X", coeff));
                    } else {
                        res.push_str(&format!(" + {}X^{}", coeff, deg));
                    }
                }
            }
        }
        return write!(f,"{}",res);
    }
}







