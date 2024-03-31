use nalgebra_glm::Vec4;

pub fn fan_triangulate<T: Copy>(vertices: &[(Vec4, T)]) -> Vec<(Vec4, T)> {
    if vertices.len() < 3 {
        return vec![];
    }
    let mut output_vertices = Vec::with_capacity(3 * (vertices.len() - 2));
    let base = 0;
    for i in 1..(vertices.len() - 1) {
        let v0 = vertices[base];
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        output_vertices.push(v0);
        output_vertices.push(v1);
        output_vertices.push(v2);
    }
    return output_vertices;
}

