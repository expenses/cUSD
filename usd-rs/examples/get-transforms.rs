use usd_rs::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let c_filename = std::ffi::CString::new(&*filename).unwrap();

    dbg!(&filename, &c_filename);

    let stage = Stage::open(&c_filename);

    let mut cache = GeomXformCache::new();

    let mesh_attributes = MeshAttributes::new();

    for prim in stage.iter_prims() {
        if let Some(order) = prim.get_attribute(&Token::new(b"xformOpOrder\0")) {
            if let Some(token_list) = order.get_token_array() {
                /*for token in &*token_list {
                    //let token = &*token;

                    dbg!(&**token);

                    if token == &Token::new(b"xformOp:translate\0") {
                    } else if token == &Token::new(b"xformOp:rotateXYZ\0") {
                    } else if token == &Token::new(b"xformOp:scale\0") {
                    } else if token == &Token::new(b"xformOp:translate:rotatePivot\0") {
                    } else if token == &Token::new(b"!invert!xformOp:translate:rotatePivot\0") {
                    } else if token == &Token::new(b"xformOp:translate:scalePivot\0") {
                    } else if token == &Token::new(b"!invert!xformOp:translate:scalePivot\0") {
                    } else if token == &Token::new(b"xformOp:transform\0") {
                    } else if token == &Token::new(b"xformOp:transform:transform\0") {
                    } else {
                        panic!();
                    }
                }*/

                dbg!(&*token_list);
            };
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
