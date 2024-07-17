mod matrices;

fn main() {
    let vec_val = vec![1,2,3,4,5,6];
    let vec_val3 = vec![2,4,5,1,2,3];
    let nrows : u32 = 3;
    let ncols : u32 = 2;
    
    let mat1 : matrices::Matrix_t<i32> = matrices::Matrix_t::new_empty(nrows,ncols);
    let mat2 : matrices::Matrix_t<i32> = matrices::Matrix_t::new(nrows,ncols,vec_val);
    let mat3 : matrices::Matrix_t<i32> = matrices::Matrix_t::new(nrows,ncols,vec_val3);
    println!("shape of the matrix : {:?}", mat1.get_shape());
    println!("shape of the matrix : {:?}", mat2.get_shape());
    println!("shape of the matrix : {:?}", mat3.get_shape());
    for i in 0..nrows {
        for j in 0..ncols {
            println!("Mat[{}][{}] = {}", i,j, mat2[(i,j)]);
        } 
    }
    println!("{}", mat1);
    println!("{}", mat2);
    println!("{}", mat3);
    println!("Sum of both : ");
    println!("{}", mat2+mat3);

    println!("{}", mat1);
    println!("{}", mat2);
    println!("{}", mat3);
}
