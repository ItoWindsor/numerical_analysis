#![allow(non_camel_case_types)]

pub use crate::common_type::CommonType_t;

pub struct Matrix_t<T : num::Num + Default + Clone + Copy> {
    nrows : u32,
    ncols : u32,
    values : Vec<T>,
}


impl<T : num::Num + Default + Clone + Copy> Matrix_t<T> {
    pub fn new(nrows : u32, ncols : u32, values : Vec<T>) -> Matrix_t<T>{
        assert_eq!(values.len() as u32, nrows*ncols);
        return Self {nrows, ncols, values};
    }

    pub fn new_empty(nrows : u32, ncols : u32) -> Self { 
        let values = vec![T::zero(); (nrows*ncols) as usize];
        return Self {nrows, ncols, values};
    }

    pub fn new_identity(size : u32) -> Self {
        let mut values = vec![T::zero(); (size*size) as usize];
        for k in 0..size    {
            values[(k*size +k) as usize] = T::one();
        }
        return Self {nrows : size, ncols : size, values}
    }

    pub fn from_array<const M: usize, const N: usize>(arr: [[T; N]; M]) -> Self {
        let nrows = M as u32;
        let ncols = N as u32;
        let mut values : Vec<T> = Vec::with_capacity((nrows * ncols) as usize);

        for row in arr.iter() {
            for elem in row.iter() {
                values.push(elem.clone());
            }
        }

        Self { nrows, ncols, values }
    }
}


// TODO : Add a method to get the shape of the matrix like np.shape() 
impl<T : num::Num + Default + Clone + Copy> Matrix_t<T> {
    pub fn get_shape(&self) -> (u32,u32) {
        let res : (u32,u32) = (self.nrows, self.ncols);
        return res;
    }
}


impl<T : num::Num + Default + Clone + Copy> Matrix_t<T> {
    fn index(&self, row : u32, col : u32) -> usize {
        return ((row*self.ncols) + col).try_into().unwrap();
    }
}

impl<T : num::Num + Default + Clone + Copy> std::ops::Index<(u32,u32)> for Matrix_t<T> {
    type Output = T;
    
    fn index(&self, index : (u32,u32)) -> &Self::Output {
        let (row,col) = index;
        return &self.values[self.index(row,col)];
    }
}

impl<T : num::Num + Default + Clone + Copy> std::ops::IndexMut<(u32,u32)> for Matrix_t<T> {

    fn index_mut(&mut self, index : (u32,u32)) -> &mut Self::Output {
        let (row,col) : (u32,u32) = index;
        let idx : usize = self.index(row,col);
        return &mut self.values[idx];
    }
}

// TODO : Implement a clone method 
impl<T : num::Num + Default + Clone + Copy> Clone for Matrix_t<T> {
    fn clone(&self) -> Self {
        return  Matrix_t {nrows : self.nrows, ncols : self.ncols, values : self.values.clone()};
    }
}


// TODO: Add a method to show the matrix 
impl<T : num::Num + Default + Clone + std::fmt::Display + Copy> std::fmt::Display for Matrix_t<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res : String = String::new();
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                res.push_str(&format!("{}",self[(i,j)])); 
                if j != self.ncols {
                    res.push_str(" ");
                }
            }
            res.push_str("\n");
        }
        return write!(f,"{}",res);
    }
}

// TODO : Add a method to sum two matrices
impl<T,U,V> std::ops::Add<&Matrix_t<U>> for &Matrix_t<T>
    where 
    T: num::Num + Default + Clone + Copy + num::NumCast,
    U: num::Num + Default + Clone + Copy + num::NumCast,
    () : CommonType_t<T,U, Output = V>,
    V: num::Num + Default + Clone + Copy + num::NumCast {
    type Output = Matrix_t<V>;

    fn add(self, mat2 : &Matrix_t<U>) -> Matrix_t<V> {
        assert!((self.nrows == mat2.nrows) & (self.ncols == mat2.ncols));
        let values: Vec<V> = self.values.iter().zip(mat2.values.iter())
            .map(|(&a, &b)| {
                let a_as_v: V = num::NumCast::from(a).unwrap();
                let b_as_v: V = num::NumCast::from(b).unwrap();
                a_as_v + b_as_v
            })
            .collect();
        return Matrix_t {nrows : self.nrows,ncols : self.ncols,values};

    }
}


// TODO : Add a method to compute product of two matrices

impl<T,U, V> std::ops::Mul<&Matrix_t<U> > for &Matrix_t<T>
    where 
    T: num::Num + Default + Clone + Copy + num::NumCast,
    U: num::Num + Default + Clone + Copy + num::NumCast,
    () : CommonType_t<T,U, Output = V>, 
    V : num::Num + Default + Clone + Copy + num::NumCast {
    type Output = Matrix_t<V>;
  
    fn mul(self, mat2 : &Matrix_t<U>) -> Matrix_t<V> {
        assert!(self.ncols == mat2.nrows);
        let nrows : u32 = self.nrows;
        let ncols : u32 = mat2.ncols;

        let values = vec![V::zero(); (nrows*ncols) as usize];
        let mut prod_mat : Matrix_t<V> = Matrix_t::new(nrows, ncols, values);
        
        for i in 0..nrows {
            for j in 0..ncols {
                for k in 0..mat2.nrows {
                    let left : V = num::NumCast::from(self[(i, k)]).unwrap() ;
                    let right : V = num::NumCast::from(mat2[(k, j)]).unwrap();
                    prod_mat[(i,j)] = prod_mat[(i,j)] + left*right;
                }
            }
        
        }
        return prod_mat;
    }
 
}
