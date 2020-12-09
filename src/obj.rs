use std::fs::File;

use std::io::{self, prelude::*, BufReader};

pub struct Mesh {
    vertices: Vec<f32>,
    vertex_normals: Vec<f32>,
    vertex_indices: Vec<usize>,
    vertex_normal_indices: Vec<usize>,
}

impl Mesh {
    pub fn empty() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            vertex_normals: Vec::new(),
            vertex_indices: Vec::new(),
            vertex_normal_indices: Vec::new()
        }
    }

    pub fn compute_faces(&self) -> Vec<f32> {
        let mut faces: Vec<f32> = Vec::new();
        assert_eq!(self.vertex_indices.len(), self.vertex_normal_indices.len());

        for f in self.vertex_indices.iter().zip(&self.vertex_normal_indices).collect::<Vec<_>>().chunks(3) {
            for (i, vni) in f {
                let j = (*i - 1) * 3;
                let x = self.vertices[j];
                let y = self.vertices[j + 1];
                let z = self.vertices[j + 2];
                faces.push(x);
                faces.push(y);
                faces.push(z);

                let j2 = (*vni - 1) * 3;
                let x2 = self.vertex_normals[j2];
                let y2 = self.vertex_normals[j2 + 1];
                let z2 = self.vertex_normals[j2 + 2];
                faces.push(x2);
                faces.push(y2);
                faces.push(z2);
            }
        }

        faces
    }

}

pub fn read_lines() -> io::Result<Mesh> {
    let file = File::open("models/on_a_plate.obj")?;
    let reader = BufReader::new(file);

    let mut mesh = Mesh::empty();

    for line in reader.lines() {
        read_line(&mut mesh, line?)?;
    }

    Ok(mesh)
}

fn read_line(mesh: &mut Mesh, line: String) -> io::Result<&Mesh> {
    let mut tokens = line.split_whitespace();
    match tokens.next() {
        Some("v") => {
            let x = tokens.next().unwrap();
            let y = tokens.next().unwrap();
            let z = tokens.next().unwrap();
            mesh.vertices.push(x.parse().unwrap());
            mesh.vertices.push(y.parse().unwrap());
            mesh.vertices.push(z.parse().unwrap());
        }
        Some("vn") => {
            let x = tokens.next().unwrap();
            let y = tokens.next().unwrap();
            let z = tokens.next().unwrap();
            mesh.vertex_normals.push(x.parse().unwrap());
            mesh.vertex_normals.push(y.parse().unwrap());
            mesh.vertex_normals.push(z.parse().unwrap());
        }
        Some("f") => {
            let p1 = tokens.next().unwrap();
            let p2 = tokens.next().unwrap();
            let p3 = tokens.next().unwrap();
            let (i1, vni1) = read_face_part(p1).unwrap();
            let (i2, vni2) = read_face_part(p2).unwrap();
            let (i3, vni3) = read_face_part(p3).unwrap();
            mesh.vertex_indices.push(i1);
            mesh.vertex_indices.push(i2);
            mesh.vertex_indices.push(i3);

            mesh.vertex_normal_indices.push(vni1);
            mesh.vertex_normal_indices.push(vni2);
            mesh.vertex_normal_indices.push(vni3);
        }
        Some(_) => { }
        None => println!("empty line?"),
    };
    Ok(mesh)
}

fn read_face_part(token: &str) -> io::Result<(usize, usize)> {
    let mut toks = token.split('/');

    let vert_index_str = toks.next().unwrap();
    let vert_index = vert_index_str.parse().unwrap();

    // discard vertex texture index
    toks.next().unwrap();

    let vert_texture_index_str = toks.next().unwrap();
    let vert_texture_index = vert_texture_index_str.parse().unwrap();
    Ok((vert_index, vert_texture_index))
}

mod tests {

    #[test]
    fn reads_file() {
        let result = super::read_lines();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn get_correct_num_verts() {
        let result = super::read_lines();
        assert_eq!(result.is_ok(), true);

        assert_eq!(result.unwrap().vertices.len(), 62081 * 3);
    }

    #[test]
    fn get_correct_first_vert() {
        let result = super::read_lines().unwrap();

        assert_eq!(result.vertices[0], 28.07870);
        assert_eq!(result.vertices[1], -157.42495);
        assert_eq!(result.vertices[2], 17.07285);
    }

    #[test]
    fn get_correct_num_faces() {
        let result = super::read_lines();
        assert_eq!(result.is_ok(), true);

        assert_eq!(result.unwrap().vertex_indices.len(), 113156 * 3); 
    }

    #[test]
    fn compute_first_face() {
        let result = super::read_lines().unwrap();

        let faces = result.compute_faces();
         
        assert_eq!(faces[0], 4.21506); 
        assert_eq!(faces[1], -157.03942); 
        assert_eq!(faces[2], 28.67336); 

        assert_eq!(faces[3], 6.24561);
        assert_eq!(faces[4], -156.80348);
        assert_eq!(faces[5], 29.33389);

        assert_eq!(faces[6], 6.24561);
        assert_eq!(faces[7],  -156.80348);
        assert_eq!(faces[8], 30.72066);

        assert_eq!(faces[0], faces[9]); 
        assert_eq!(faces[1], faces[10]); 
        assert_eq!(faces[2], faces[11]); 

    }

}
