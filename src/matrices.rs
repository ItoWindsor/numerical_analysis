pub struct Matrix_t<T : num::Num + Default + Clone> {
    nrows : u32,
    ncols : u32,
    values : Vec<T>,
}


impl<T : num::Num + Default + Clone> Matrix_t<T> {
    pub fn new(nrows : u32, ncols : u32, values : Vec<T>) -> Matrix_t<T>{
        assert_eq!(values.len() as u32, nrows*ncols);
        return Self {nrows, ncols, values};
    }
}

impl<T : num::Num + Default + Clone> Matrix_t<T> {
    pub fn new_empty(nrows : u32, ncols : u32) -> Self { 
    let values = vec![T::default(); (nrows*ncols) as usize];
    return Self {nrows, ncols, values};
    }
}

// TODO Add a method to get the shape of the matrix like np.shape() 
impl<T : num::Num + Default + Clone> Matrix_t<T> {
    pub fn get_shape(&self) -> (u32,u32) {
        let res : (u32,u32) = (self.nrows, self.ncols);
        return res;
    }
}


impl<T : num::Num + Default + Clone> Matrix_t<T> {
    fn index(&self, row : u32, col : u32) -> usize {
        return ((row*self.ncols) + col).try_into().unwrap();
    }
}

impl<T : num::Num + Default + Clone> std::ops::Index<(u32,u32)> for Matrix_t<T> {
    type Output = T;
    
    fn index(&self, index : (u32,u32)) -> &Self::Output {
        let (row,col) = index;
        return &self.values[self.index(row,col)];
    }
}


// TODO : Add a method to create a matrix from an array of array ( [ [0,1,2],[3,4,5],[6,7,8]  ] )

// TODO Add a method to show the matrix 

impl<T : num::Num + Default + Clone + std::fmt::Debug> std::fmt::Display for Matrix_t<T> {
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


// TODO Add a method to sum two matrices

//impl<T> std::ops::Add<Matrices_t<T> > for Matrices_t<T> {
//    type Output = Matrices_t<T>;
//    fn add(self, mat2 : Matrices_t<T>) -> Matrices_t<T> {
//       //assert!(self.nrows == mat2.nrows & self.ncols == mat2.ncols); 
//        
//    }
//}

// TODO Add a method to compute product of two matrices
//
// impl<T> std::ops::Mul<Matrices_t<T> > for Matrices_t<T> {
//  type Output = Matrices_t<T>;
//  
//  fn mul(self, mat2 : Matrices_t<T>) -> Matrices_t<T> {
//      // assert!(self.ncols == mat2.nrows);
//  }
// }
