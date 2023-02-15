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
        for attr in &*prim.get_attributes() {
            if matches!(
                attr.get_type_name().to_bytes(),
                b"GfMatrix4d"
                    | b"VtArray<TfToken>"
                    | b"SdfAssetPath"
                    | b"VtArray<GfQuath>"
                    | b"GfVec4f"
                    | b"VtArray<GfQuatf>"
                    | b"VtArray<GfVec4f>"
                    | b"VtArray<std::string>"
                    | b"VtArray<double>"
                    | b"GfQuatf"
            ) {
                dbg!(attr.get_type_name());
                continue;
            }
            if attr.get_value().is_none() {
                println!("{}:{}", &*attr.get_namespace(), &*attr.get_base_name());
                panic!();
            }
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
