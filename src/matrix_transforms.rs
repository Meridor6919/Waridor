pub const IDENTITY_MATRIX : [[f32; 4]; 4] = [[1.0, 0.0, 0.0, 0.0],[0.0, 1.0, 0.0, 0.0],[0.0, 0.0, 1.0, 0.0],[0.0, 0.0, 0.0, 1.0]];

pub fn rotate_around_x_axis(angle : f32) -> [[f32; 4];4]{
    let matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin(), 0.0],
        [0.0, angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];
    return matrix;
}
pub fn rotate_around_y_axis(angle : f32) -> [[f32; 4];4]{
    let matrix = [
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];
    return matrix;
}
pub fn rotate_around_z_axis(angle : f32) -> [[f32; 4];4]{
    let matrix = [
        [ angle.cos(), angle.sin(), 0.0, 0.0],
        [-angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];
    return matrix;
}
pub fn translate(v : [f32; 3]) -> [[f32; 4];4]{
    let matrix = [
        [ 1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [v[0], v[1], v[2], 1.0f32]
    ];
    return matrix;
}

