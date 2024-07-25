use std::collections::HashMap;

mod common_type;
mod matrices;
mod linear_system;
mod polynome;

use polynome::Polynome_t;
use matrices::Matrix_t;


fn main() {

    let id_matrix1 : Matrix_t<i32> = Matrix_t::new_identity(3);
    
    let arr = [[0;3], [1;3], [2;3]];
    let arr2 = [[1.1, 2.5, 5.6],[1.0;3], [3.0;3]];
    
    let mat_arr = Matrix_t::from_array(arr);
    let mat_arr2 = Matrix_t::from_array(arr2); 
   
    let prod_mat1 = &id_matrix1 * &mat_arr2;
    let prod_mat2 = &mat_arr * &id_matrix1; 
   
    println!("{}", id_matrix1);
    println!("{}",mat_arr2);
    println!("{}", prod_mat1);
    println!("{}", prod_mat2);

    let deg_to_coeff1 : HashMap<u32, f64> = HashMap::from([(0, 2.5),(1,3.0),(3,5.0)]);
    let deg_to_coeff2 : HashMap<u32, f64> = HashMap::from([(0, 2.5),(1,-3.0),(3,5.0)]);
    let p1 : Polynome_t<f64> = Polynome_t::new(&deg_to_coeff1);
    let p2 : Polynome_t<f64> = Polynome_t::new(&deg_to_coeff2);
    
    let arr_deg = [0,1,58];
    let arr_coeff = [-10,-4, 67];
    let p3 : Polynome_t<i32> = Polynome_t::from_array(arr_deg,arr_coeff);
    
    println!("{}",p1);
    println!("{}",p2);
    println!("{}",p3);
}
