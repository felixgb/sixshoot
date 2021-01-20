use std::fs::File;

use std::io::{self, prelude::*, BufReader};

pub struct Mesh {
    vertices: Vec<f32>,
    vertex_normals: Vec<f32>,
    texture_coords: Vec<f32>,
    face_indices: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn empty() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            vertex_normals: Vec::new(),
            texture_coords: Vec::new(),
            face_indices: Vec::new(),
        }
    }

    pub fn compute_faces(&self) -> Vec<f32> {
        let mut faces: Vec<f32> = Vec::new();

        for f in self.face_indices.chunks(3) {
            for (i, tci, vni) in f {
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

                let j3 = (*tci - 1) * 2;
                let x3 = self.texture_coords[j3];
                let y3 = self.texture_coords[j3 + 1];
                faces.push(x3);
                faces.push(y3);
            }
        }

        faces
    }

}

pub fn read_lines(obj_file_path: &str) -> io::Result<Mesh> {
    let file = File::open(obj_file_path)?;
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
        Some("vt") => {
            let u = tokens.next().unwrap();
            let v = tokens.next().unwrap();
            mesh.texture_coords.push(u.parse().unwrap());
            mesh.texture_coords.push(v.parse().unwrap());
        }
        Some("f") => {
            let p1 = tokens.next().unwrap();
            let p2 = tokens.next().unwrap();
            let p3 = tokens.next().unwrap();
            let f1 = read_face_part(p1).unwrap();
            let f2 = read_face_part(p2).unwrap();
            let f3 = read_face_part(p3).unwrap();
            mesh.face_indices.push(f1);
            mesh.face_indices.push(f2);
            mesh.face_indices.push(f3);
        }
        Some(_) => { }
        None => println!("empty line?"),
    };
    Ok(mesh)
}

fn read_face_part(token: &str) -> io::Result<(usize, usize, usize)> {
    let mut toks = token.split('/');

    let vert_index_str = toks.next().unwrap();
    let vert_index = vert_index_str.parse().unwrap();

    let texture_coord_index_str = toks.next().unwrap();
    let texture_coord_index = texture_coord_index_str.parse().unwrap();

    let vert_texture_index_str = toks.next().unwrap();
    let vert_texture_index = vert_texture_index_str.parse().unwrap();
    Ok((vert_index, texture_coord_index, vert_texture_index))
}

mod tests {

    #[test]
    fn reads_file() {
        let result = super::read_lines("assets/floor.obj");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn get_correct_num_verts() {
        let result = super::read_lines("assets/floor.obj");
        assert_eq!(result.is_ok(), true);

        assert_eq!(result.unwrap().vertices.len(), 62081 * 3);
    }

    #[test]
    fn get_correct_first_vert() {
        let result = super::read_lines("assets/floor.obj").unwrap();

        assert_eq!(result.vertices[0], 28.07870);
        assert_eq!(result.vertices[1], -157.42495);
        assert_eq!(result.vertices[2], 17.07285);
    }

    #[test]
    fn compute_first_face() {
        let result = super::read_lines("assets/floor.obj").unwrap();

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
