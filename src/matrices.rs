pub struct Matrices_t<T> {
    nrows : usize,
    ncols : usize,
    values : Vec<T>,
}

impl<T> Matrices_t<T> {
    pub fn new(nrows : usize, ncols : usize, values : Vec<T>) -> Self {
        return Self { nrows : nrows, ncols : ncols, values : values};
    }
}

// TODO Add a method to get values like Mat[i][k]

//impl<T> std::ops::Index<usize> for Matrices_t<T> {
//    type Output = [T];
//
//    fn index(&self, row_index : usize) -> &Self::Output {
//        assert!(row_index < self.nrows);
//        let start = row_index * self.nrows;
//        let end = start + self.ncols;
//        return &self.values[start..end];
//    }
//}
//



// TODO Add a method to show the matrix 

// //impl<T> std::fmt::Display for Matrices_t<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let mut res : String;
//        for i in 0..self.nrows-1 {
//            for j in 0..self.ncols-1 {
//                res.push_str(self.values[self.nrows*i + k*self.ncols]); 
//            }
//            res.push_str("\n");
//        }
//        return write!(f, "test matrices");
//    }
//}


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
