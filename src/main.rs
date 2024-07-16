mod matrices;

fn main() {
    let vec_val = vec![1,2,3,4,5,6];
    let nrows : usize = 3;
    let ncols : usize = 2;
    
    let mat : matrices::Matrices_t<i32> = matrices::Matrices_t::new(nrows,ncols,vec_val);

    //println!("{}",mat[1][0]);
    //println!("{mat}");
}
