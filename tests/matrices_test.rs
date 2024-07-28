#[cfg(test)]
mod tests{
    
    use SciRust::matrices;
    use matrices::Matrix_t;

    #[test]
    fn good_size(){
    let mat : Matrix_t<i32> = Matrix_t::new_empty(3,3);
    assert_eq!(mat.get_shape(),(3,3));
    assert_eq!(mat.values.len(),9 as usize);
    }

    #[test]
    fn creating_identity() {
        let id_matrix1 : Matrix_t<f64> = Matrix_t::new_identity(3);
        let id_mat : Matrix_t<f64> = Matrix_t::from_array([[1.0,0.0,0.0],[0.0, 1.0, 0.0],[0.0, 0.0, 1.0]]);
        assert_eq!(id_matrix1,id_mat);
    }
}
