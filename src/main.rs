mod matrices;
mod linear_system;

fn main() {
    let id_matrix1 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    let id_matrix2 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    
    let arr = [[0;3], [1;3], [2;3]];
    let arr2 = [[1.1, 2.5, 5.6],[1.0;3], [3.0;3]];
    
    let mat_arr = matrices::Matrix_t::from_array(arr);
    let mat_arr2 = matrices::Matrix_t::from_array(arr2); 
    let mat_id : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    
    let prod_mat2 = &mat_id * &mat_arr2;
   
    println!("{}", mat_id);
    println!("{}",mat_arr2);
    println!("{}", prod_mat2);
}
