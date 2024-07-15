pub struct Matrices_t<T> {
    nrows : u32,
    ncols : u32,
    values : Vec<T>,
}

impl<T> Matrices_t<T> {
    pub fn new(nrows : u32, ncols : u32, values : Vec<T>) -> Self {
    return Self { nrows : nrows, ncols : ncols, values : values};
    }
}

impl<T> std::fmt::Display for Matrices_t<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "test matrices");
    }
}
