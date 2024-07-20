mod matrices;

fn main() {
    let arr = [[0;4], [1;4], [1,2,3,4]];
    println!("array of array : ");
    println!("{:?}", arr);
    println!("len : {}", arr.len());
    let nrows : u32 = arr.len() as u32;
    let ncols : u32 = arr[0].len() as u32;
    let mut vec : Vec<i32>= Vec::new();

    for k in 0..nrows as usize{
        for j in 0..ncols as usize{
            vec.push(arr[k][j]);
        }
    }
    println!("{:?}",vec);

    let mat_from_arr = matrices::Matrix_t::from_array(arr);
    println!("matrix from arr");
    println!("{}",mat_from_arr);
    
}
