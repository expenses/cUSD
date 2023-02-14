mod bindings;
mod rust;
use std::mem::ManuallyDrop;

use rust::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let c_filename = std::ffi::CString::new(&*filename).unwrap();

    dbg!(&filename, &c_filename);

    let stage = Stage::open(&c_filename);

    let mut cache = GeomXformCache::new();

    let mesh_attributes = MeshAttributes::new();

    for prim in stage.iter_prims() {
        let type_name = prim.get_type_name();

        if type_name.to_bytes() == b"Sphere" {
            let display_colours = prim
                .get_attribute(&Token::new(b"primvars:displayColor\0"))
                .unwrap()
                .get_vec3f_array();
            dbg!(&*display_colours);
        } else if type_name.to_bytes() == b"Mesh" {
            for _ in 0..10000 {
                for prim in &*prim.get_all_subsets() {
                    let mat = prim.compute_bound_material();

                    let mut source = match mat.compute_surface_shader() {
                        Some(source) => source,
                        None => continue,
                    };

                    let diffuse_input = source.get_input(&Token::new(b"diffuseColor\0")).unwrap();
                    let connected_sources = diffuse_input.get_all_connected_sources();
                    let slice = connected_sources;
                    source = slice[0].source();

                    let texture_input = source.get_input(&Token::new(b"file\0")).unwrap();
                    let string = texture_input.get_resolved_path().unwrap();
                    if !string.is_empty() {
                        dbg!(&*string);
                    }
                }

                let mat = prim.compute_bound_material();

                let mut source = match mat.compute_surface_shader() {
                    Some(source) => source,
                    None => continue,
                };

                let diffuse_input = source.get_input(&Token::new(b"diffuseColor\0")).unwrap();
                let connected_sources = diffuse_input.get_all_connected_sources();
                source = connected_sources[0].source();

                let texture_input = source.get_input(&Token::new(b"file\0")).unwrap();
                if let Some(string) = texture_input.get_resolved_path() {
                    dbg!(&*string);
                }
                //let prim = diffuse_input.get_prim();
                //dbg!(&prim.get_type_name());

                let vertex_counts = prim
                    .get_attribute(&mesh_attributes.face_vertex_counts)
                    .unwrap()
                    .get_int_array();
                let vertex_indices = prim
                    .get_attribute(&mesh_attributes.face_vertex_indices)
                    .unwrap()
                    .get_int_array();
                let uvs = Some(prim.get_attribute(&mesh_attributes.uv_map).unwrap())
                    .map(|x| x.get_vec2f_array());
                let normals = prim.get_attribute(&mesh_attributes.normals).unwrap();
                let points = prim
                    .get_attribute(&mesh_attributes.points)
                    .unwrap()
                    .get_vec3f_array();

                for _ in 0..1000 {
                    //let uvs = prim.get_attribute(&mesh_attributes.st).unwrap().get_vec2f_array();
                }

                let x = {
                    normals
                        .get_token_metadata(&Token::new(b"interpolation\0"))
                        .as_ref()
                        == Some(&mesh_attributes.face_varying)
                };
                assert_eq!(vertex_indices.len(), normals.get_vec3f_array().len() / 3);
                assert_eq!(vertex_indices.len(), uvs.as_ref().unwrap().len() / 2);
                dbg!(
                    vertex_counts.len(),
                    vertex_indices.len(),
                    uvs.map(|x| x.len() / 2),
                    normals.get_vec3f_array().len() / 3,
                    points.len() / 3
                );

                for subset in &*prim.get_all_subsets() {
                    dbg!(subset.get_indices_attr().get_int_array().len());
                }
            }
        } else {
            //dbg!(&type_name);
        }
    }

    for layer in &*stage.get_used_layers() {
        if let Some(path) = layer.get_path() {
            //dbg!(&*path, layer.reload());
        }
    }
}

struct MeshAttributes {
    face_vertex_counts: Token,
    face_vertex_indices: Token,
    uv_map: Token,
    st: Token,
    normals: Token,
    points: Token,
    face_varying: Token,
}

impl MeshAttributes {
    fn new() -> Self {
        Self {
            face_vertex_counts: Token::new(b"faceVertexCounts\0"),
            face_vertex_indices: Token::new(b"faceVertexIndices\0"),
            uv_map: Token::new(b"primvars:UVMap\0"),
            normals: Token::new(b"normals\0"),
            st: Token::new(b"st\0"),
            points: Token::new(b"points\0"),
            face_varying: Token::new(b"faceVarying\0"),
        }
    }
}
