mod matrices;
mod linear_system;

fn main() {
    let id_matrix1 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    let id_matrix2 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    
    let arr = [[0;3], [1;3], [2;3]];
    let arr2 = [[0.1, 2.5, 5.6],[1.0;3]];
    let mat_arr = matrices::Matrix_t::from_array(arr);
    let mat_arr2 = matrices::Matrix_t::from_array(arr2); 
    let mat3 = &id_matrix1 * &id_matrix2;
    let mat4 = &mat_arr * &mat_arr;
    let mat5 = &mat_arr2 * &mat_arr;

    println!("arr = {:?}", arr);
    println!("arr2 = {:?}", arr2);
    println!("mat from arr : \n{}",mat_arr);
    println!("mat from arr2 : \n{}", mat_arr2);


    println!("{}",mat3);
    println!("{}",mat4);
    println!("{}", mat5);
}
