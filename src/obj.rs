use std::fs::File;

use std::io::{self, prelude::*, BufReader};

pub struct Mesh {
    vertices: Vec<f32>,
    face_indices: Vec<usize>,
}

impl Mesh {
    pub fn empty() -> Mesh {
        Mesh { vertices: Vec::new(), face_indices: Vec::new() }
    }

    pub fn compute_faces(&self) -> Vec<f32> {
        let mut faces: Vec<f32> = Vec::new();

        for f in self.face_indices.chunks(3) {
            for i in f {
                let j = (*i - 1) * 3;
                let x = self.vertices[j];
                let y = self.vertices[j + 1];
                let z = self.vertices[j + 2];
                faces.push(x);
                faces.push(y);
                faces.push(z);
            }
        }

        faces
    }

}

pub fn read_lines() -> io::Result<Mesh> {
    let file = File::open("models/felix.obj")?;
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
        Some("f") => {
            let p1 = tokens.next().unwrap();
            let p2 = tokens.next().unwrap();
            let p3 = tokens.next().unwrap();
            let i1 = read_face_part(p1).unwrap();
            let i2 = read_face_part(p2).unwrap();
            let i3 = read_face_part(p3).unwrap();
            mesh.face_indices.push(i1);
            mesh.face_indices.push(i2);
            mesh.face_indices.push(i3);
        }
        Some(_) => { }
        None => println!("asdasdasd"),
    };
    Ok(mesh)
}

fn read_face_part(token: &str) -> io::Result<usize> {
    let vert_index_str = token.split('/').next().unwrap();
    let vert_index = vert_index_str.parse().unwrap();
    Ok(vert_index)
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

        assert_eq!(result.unwrap().face_indices.len(), 113156 * 3); 
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
