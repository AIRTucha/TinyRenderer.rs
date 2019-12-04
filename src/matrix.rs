pub struct Matrix4x4(pub [[f64; 4]; 4]);

impl Matrix4x4 {
    pub fn new(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Matrix4x4 {
        Matrix4x4([row0, row1, row2, row3])
    }
}
