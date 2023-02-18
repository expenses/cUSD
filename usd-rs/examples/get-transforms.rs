use usd_rs::*;
use std::collections::HashMap;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let c_filename = std::ffi::CString::new(&*filename).unwrap();

    dbg!(&filename, &c_filename);

    let stage = Stage::open(&c_filename);

    let mut cache = GeomXformCache::new();
    let mut getter = XformGetter::default();

    let mesh_attributes = MeshAttributes::new();

    let mut i = 0;

    for prim in stage.iter_prims() {
        let xform = getter.get_transform(&prim);
        //if prim.get_type_name().to_bytes() != b"Xform" {
            let cache_xform = glam::DMat4::from_cols_array(&cache.get_transform(&prim));
            if dbg!(sq_err(cache_xform, xform)) > 5.0e-7 {
                dbg!(cache_xform, xform);
                panic!();
            }
        //}
        i+= 1;
    }

    dbg!(i);

    for layer in &*stage.get_used_layers() {
        if let Some(path) = layer.get_path() {
            //dbg!(&*path, layer.reload());
        }
    }
}

#[derive(Default)]
struct XformGetter {
    inner: HashMap<usize, glam::DMat4>,
}

impl XformGetter {
    fn get_transform(&self, prim: &Prim) -> glam::DMat4 {
        let mut transform = self.get_local_transform(prim);

        let mut parent = prim.get_parent();
        while let Some(parent_prim) = parent {
            transform = self.get_local_transform(&parent_prim) * transform;
            parent = parent_prim.get_parent();
        }

        transform
    }

    fn get_dvec3(&self, attr: &Attribute) -> glam::DVec3 {
        if attr.get_type_name().to_bytes() == b"GfVec3f" {
            glam::Vec3::from(attr.get_vec3f().unwrap()).as_dvec3()
        } else {
            glam::DVec3::from(attr.get_vec3d().unwrap())
        }
    }

    fn get_local_transform(&self, prim: &Prim) -> glam::DMat4 {
        let mut transform = glam::DMat4::IDENTITY;
        let mut rotate_pivot = None;//glam::DVec3::ZERO;
        let mut scale_pivot: Option<glam::DVec3> = None;

        //dbg!(&*prim.get_type_name());

        if let Some(order) = prim.get_attribute(&Token::new(b"xformOpOrder\0")) {
            if let Some(token_list) = order.get_token_array() {
                for token in &*token_list {
                    match &**token {
                        "xformOp:transform" | "xformOp:transform:transform" => {
                            let attr = prim.get_attribute(token).unwrap();

                            let matrix = glam::DMat4::from_cols_array(&attr.get_matrix4d().unwrap());
                            transform *= matrix;
                        },
                        "xformOp:translate" => {
                            let attr = prim.get_attribute(token).unwrap();

                            let translation = self.get_dvec3(&attr);
                            transform *= glam::DMat4::from_translation(translation);
                        }
                        "xformOp:rotateY" => {
                            assert!(rotate_pivot.is_none());

                            let attr = prim.get_attribute(token).unwrap();

                            let rotation_y = attr.get_double().unwrap().to_radians();
                            transform *= glam::DMat4::from_rotation_y(rotation_y);
                        },
                        "xformOp:rotateZYX" | "xformOp:rotateZYX:rotate" => {
                            assert!(rotate_pivot.is_none());

                            let attr = prim.get_attribute(token).unwrap();

                            let rotation_xyz = glam::DVec3::from(attr.get_vec3d().unwrap());
                            transform *= glam::DMat4::from_euler(glam::EulerRot::XYZ, rotation_xyz.x.to_radians(), rotation_xyz.y.to_radians(), rotation_xyz.z.to_radians());
                        },
                        "xformOp:rotateZXY"| "xformOp:rotateZXY:rotate" => {
                            assert!(rotate_pivot.is_none());

                            let attr = prim.get_attribute(token).unwrap();

                            let rotation_xyz = self.get_dvec3(&attr);
                            transform *= glam::DMat4::from_euler(glam::EulerRot::YXZ, rotation_xyz.y.to_radians(), rotation_xyz.x.to_radians(), rotation_xyz.z.to_radians());
                        },
                        "xformOp:rotateXYZ" => {
                            assert!(rotate_pivot.is_none());

                            let attr = prim.get_attribute(token).unwrap();

                            let rotation_xyz = self.get_dvec3(&attr);
                            transform *= glam::DMat4::from_euler(glam::EulerRot::ZYX, rotation_xyz.z.to_radians(), rotation_xyz.y.to_radians(), rotation_xyz.x.to_radians());
                        },
                        "xformOp:rotateYXZ" => {
                            assert!(rotate_pivot.is_none());

                            let attr = prim.get_attribute(token).unwrap();

                            let rotation_xyz = self.get_dvec3(&attr);
                            transform *= glam::DMat4::from_euler(glam::EulerRot::ZXY, rotation_xyz.z.to_radians(), rotation_xyz.x.to_radians(), rotation_xyz.y.to_radians());
                        },
                        "xformOp:scale" => {
                            if let Some(pivot) = scale_pivot {
                                //transform *= glam::DMat4::from_translation(-pivot);
                            }

                            let attr = prim.get_attribute(token).unwrap();

                            let scale = self.get_dvec3(&attr);
                            transform *= glam::DMat4::from_scale(scale);

                            if let Some(pivot) = scale_pivot {
                                //transform *= glam::DMat4::from_translation(pivot);
                            }
                        }
                        "xformOp:translate:rotatePivot" => {
                            rotate_pivot = Some(transform.w_axis.truncate());
                            dbg!(rotate_pivot);
                        }
                        "!invert!xformOp:translate:rotatePivot" => {
                            rotate_pivot = None;
                        }
                        "xformOp:translate:scalePivot" => {
                            scale_pivot = Some(transform.w_axis.truncate());
                            dbg!(scale_pivot);
                        }
                        "!invert!xformOp:translate:scalePivot" => {
                            scale_pivot = None;
                        }
                        "xformOp:orient:orient" => {
                            let attr = prim.get_attribute(token).unwrap();
                            if attr.get_type_name().to_bytes() == b"GfQuatf" {
                                let quat = glam::Quat::from_array(attr.get_quatf().unwrap()).as_f64();
                                transform *= glam::DMat4::from_quat(quat);
                            } else {
                                panic!()
                            }
                        }
                        _ => {
                            dbg!(&**token);
                            panic!()
                        }
                    }
                }
            }
        }

        transform
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

fn sq_err(a: glam::DMat4, b: glam::DMat4) -> f64 {
    let mut err = 0.0;
    for (a, b) in a.to_cols_array().iter().zip(&b.to_cols_array()) {
        err += (a - b).abs();// * (a - b).abs()
    }
    err / 16.0
}
