mod matrices;

fn main() {
    let vec_val = vec![1,2,3,4,5,6];
    let vec_val3 = vec![2,4,5,1,2,3];
    let nrows : u32 = 3;
    let ncols : u32 = 2;
  
    let arr = [10,0];
    println!("{:?}",arr);
    let mat1 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_empty(nrows,ncols);
    let mat2 : matrices::Matrix_t<i32> = matrices::Matrix_t::new(nrows,ncols,vec_val);
    let mat3 : matrices::Matrix_t<i32> = matrices::Matrix_t::new(nrows,ncols,vec_val3);
    println!("mat1 = \n{}", mat1);
    println!("mat2 = \n{}", mat2);
    println!("mat3 = \n{}", mat3);
    println!("Sum of both : ");
    let mat_temp = &mat2 + &mat3;
    let mat_temp2 = &mat3 + &mat2;
    println!("{}", mat_temp); 
    println!("{}", mat_temp2);

    println!("mat3 = \n{}", mat3); 
}
