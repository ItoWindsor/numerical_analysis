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
        let mut values = Vec::with_capacity((nrows * ncols) as usize);

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


// TODO : Add a method to create a matrix from an array of array ( [ [0,1,2],[3,4,5],[6,7,8]  ] )


// TODO : Implement a clone method 
impl<T : num::Num + Default + Clone + Copy> Clone for Matrix_t<T> {
    fn clone(&self) -> Self {
        return  Matrix_t {nrows : self.nrows, ncols : self.ncols, values : self.values.clone()};
    }
}


// TODO: Add a method to show the matrix 
impl<T : num::Num + Default + Clone + std::fmt::Debug + Copy> std::fmt::Display for Matrix_t<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res : String = String::new();
        for i in 0..self.nrows {
            for j in 0..self.ncols {
               res.push_str(&format!("{:?}",self[(i,j)])); 
            }
            res.push_str("\n");
        }
        return write!(f,"{}",res);
    }
}

// TODO : Add a method to sum two matrices
impl<T : num::Num + Default + Clone + Copy> std::ops::Add<&Matrix_t<T> > for &Matrix_t<T> {
    type Output = Matrix_t<T>;
    fn add(self, mat2 : &Matrix_t<T>) -> Matrix_t<T> {
        assert!((self.nrows == mat2.nrows) & (self.ncols == mat2.ncols));
        let vec1 : Vec<T> = self.values.clone();
        let vec2 : Vec<T> = mat2.values.clone();
        let new_vec : Vec<T> = vec1.into_iter().zip(vec2.into_iter()).map(|(a,b)| a+b).collect();
       return Matrix_t {nrows : self.nrows, ncols : self.ncols, values : new_vec};
    }
}

// TODO : Add a method to compute product of two matrices
//
// impl<T> std::ops::Mul<Matrices_t<T> > for Matrices_t<T> {
//  type Output = Matrices_t<T>;
//  
//  fn mul(self, mat2 : Matrices_t<T>) -> Matrices_t<T> {
//      // assert!(self.ncols == mat2.nrows);
//  }
// }
