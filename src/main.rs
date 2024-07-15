mod matrices;

fn main() {
    let vec_val = vec![1,2,3];
    let nrows : u32 = 3;
    let ncols : u32 = 1;

    let mat : matrices::Matrices_t<i32> = matrices::Matrices_t::new(nrows,ncols,vec_val);
    println!("{mat}");
}
