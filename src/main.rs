mod matrices;

fn main() {
    let vec_val = vec![1,1,1,1,1,1,1,1,1];
    let nrows : u32 = 3;
    let ncols : u32 = 3;
    let mat1 = matrices::Matrix_t::new(nrows,ncols,vec_val);
    println!("matrix of One = \n{}", mat1);
    let mat_identity : matrices::Matrix_t<i32> = matrices::Matrix_t::new_identity(3);
    println!("identity matrix = \n{}", mat_identity);
    let sum_mat = &mat_identity + &mat1;
    println!("sum matrix = \n{}", sum_mat);
}
