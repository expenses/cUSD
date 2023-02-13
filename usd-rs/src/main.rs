mod bindings;
use std::mem::ManuallyDrop;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let c_filename = std::ffi::CString::new(&*filename).unwrap();

    dbg!(&filename, &c_filename);

    let stage = Stage::open(&c_filename);

    let mut cache = XformCache::new();

    const X: &'static [u8] = b"faceVarying\0";

    let ex = Token::new(X);


    for prim in stage.iter_prims() {
        let type_name = prim.get_type_name();

        if type_name.to_bytes() == b"Sphere" {
            let display_colours = prim
                .get_attribute(&Token::new(b"primvars:displayColor\0")).unwrap()
                .get_vec3f_array();
            dbg!(&display_colours.slice());
        } else if type_name.to_bytes() == b"Mesh" {
            /*for prim in prim.get_all_subsets().slice() {
                let mat = prim.compute_bound_material();

                let mut source = match mat.compute_surface_shader() {
                    Some(source) => source,
                    None => continue
                };
    
                let diffuse_input = source.get_input(b"diffuseColor\0");
                let connected_sources = diffuse_input.get_all_connected_sources();
                let slice = connected_sources.slice();
                source = slice[0].source();

                let texture_input = source.get_input(b"file\0");
                let string = texture_input.get_resolved_path();
                if !string.str().is_empty() { 
                    //dbg!(&string.str());
                }
            }*/

            let mat = prim.compute_bound_material();


            let mut source = match mat.compute_surface_shader() {
                Some(source) => source,
                None => continue
            };

            let diffuse_input = source.get_input(&Token::new(b"diffuseColor\0"));
            let connected_sources = diffuse_input.get_all_connected_sources();
            let slice = connected_sources.slice();
            source = slice[0].source();

            let texture_input = source.get_input(&Token::new(b"file\0"));
            let string = texture_input.get_resolved_path();
                if !string.str().is_empty() { 
                    dbg!(&string.str());
                }
            //let prim = diffuse_input.get_prim();
            //dbg!(&prim.get_type_name());

            let vertex_counts = prim.get_attribute(&Token::new(b"faceVertexCounts\0")).unwrap().get_int_array();
            let vertex_indices = prim.get_attribute(&Token::new(b"faceVertexIndices\0")).unwrap().get_int_array();
            let uvs = prim.get_attribute(&Token::new(b"primvars:UVMap\0")).is_some();//.get_vec2f_array();
            let normals = prim.get_attribute(&Token::new(b"normals\0")).unwrap();//.get_vec3f_array();
            let points = prim.get_attribute(&Token::new(b"points\0")).unwrap().get_vec3f_array();

            //dbg!(normals.get_token_metadata(&Token::new(b"interpolation\0")).unwrap().equal(&ex));
            //dbg!(vertex_counts.slice().len(), vertex_indices.slice().len(), uvs.slice().len() / 2, normals.slice().len() / 3, points.slice().len() / 3);

            for subset in prim.get_all_subsets().slice() {
                //dbg!(subset.get_indices_attr().get_int_array().slice().len());
            }
        } else {
            //dbg!(&type_name);
        }
    }
}

struct Stage {
    inner: *mut bindings::cusd_Stage,
}

impl Stage {
    fn open(filename: &std::ffi::CStr) -> Self {
        Self {
            inner: unsafe {
                bindings::cusd_Stage_open(filename.as_ptr())
            }
        }
    }

    fn iter_prims(&self) -> PrimIterator {
        PrimIterator {
            inner: unsafe { bindings::cusd_Stage_iter_prims(self.inner) },
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
struct ShadeSourceInfo {
    inner: bindings::cusd_ShadeConnectionSourceInfo,
}

impl ShadeSourceInfo {
    fn source(&self) -> Shader {
        Shader {
            inner: unsafe { bindings::cusd_ShadeConnectionSourceInfo_source(&self.inner) },
        }
    }
}

struct ShadeSourceInfoVector {
    inner: bindings::cusd_ShadeSourceInfoVector,
}

impl ShadeSourceInfoVector {
    fn slice<'a>(&'a self) -> &'a [ShadeSourceInfo] {
        let size = unsafe { bindings::cusd_ShadeSourceInfoVector_size(&self.inner) };

        let pointer = unsafe { bindings::cusd_ShadeSourceInfoVector_pointer(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer as *const _, size) }
    }
}

#[derive(Debug)]
struct CusdString {
    inner: bindings::cusd_String,
}

impl CusdString {
    fn str<'a>(&'a self) -> &'a str {
        let pointer = unsafe { bindings::cusd_String_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_String_size(&self.inner) };

        std::str::from_utf8(unsafe { std::slice::from_raw_parts(pointer as *const u8, size) })
            .unwrap()
    }
}

struct ShadeInput {
    inner: bindings::cusd_ShadeInput,
}

impl ShadeInput {
    fn get_all_connected_sources(&self) -> ShadeSourceInfoVector {
        ShadeSourceInfoVector {
            inner: unsafe { bindings::cusd_ShadeInput_get_connected_sources(&self.inner) },
        }
    }

    fn get_resolved_path(&self) -> CusdString {
        CusdString {
            inner: unsafe { bindings::cusd_ShadeInput_get_resolved_path(&self.inner) },
        }
    }
}

struct Shader {
    inner: bindings::cusd_ShadeShader,
}

impl Shader {
    fn get_input(&self, name: &Token) -> ShadeInput {
        ShadeInput {
            inner: unsafe {
                bindings::cusd_ShadeShader_get_input(&self.inner, &name.inner)
            },
        }
    }
}

struct Material {
    inner: bindings::cusd_ShadeMaterial,
}

impl Material {
    fn compute_surface_shader(&self) -> Option<Shader> {
        let mut shader = Default::default();

        let valid = unsafe {
            bindings::cusd_ShadeMaterial_compute_surface_source(&self.inner, &mut shader)
        };

        if valid {
            Some(Shader { inner: shader })
        } else {
            None
        }
    }
}

#[repr(transparent)]
struct Subset {
    inner: bindings::cusd_GeomSubset,
}

impl Subset {
    fn get_indices_attr(&self) -> Attribute {
        Attribute {
            inner: unsafe { bindings::cusd_GeomSubset_get_indices_attr(&self.inner) },
        }
    }

    fn compute_bound_material(&self) -> Material {
        Material {
            inner: unsafe { bindings::cusd_GeomSubset_compute_bound_material(&self.inner) },
        }
    }
}

struct PrimIterator {
    inner: bindings::cusd_PrimRange,
}

impl Iterator for PrimIterator {
    type Item = Prim;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if bindings::cusd_PrimRange_is_empty(&self.inner) {
                return None;
            }

            return Some(Prim {
                inner: bindings::cusd_PrimRange_next(&mut self.inner),
            });
        }
    }
}

struct Subsets {
    inner: bindings::cusd_GeomSubsetVector,
}

impl Subsets {
    fn slice<'a>(&'a self) -> &'a [Subset] {
        let pointer = unsafe { bindings::cusd_GeomSubsetVector_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_GeomSubsetVector_size(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer as *const _, size) }
    }
}

struct Vec3fArray {
    inner: bindings::cusd_Vec3fArray,
}

impl Vec3fArray {
    fn slice<'a>(&'a self) -> &'a [f32] {
        let pointer = unsafe { bindings::cusd_Vec3fArray_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_Vec3fArray_size(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer, size * 3) }
    }
}

struct Vec2fArray {
    inner: bindings::cusd_Vec2fArray,
}

impl Vec2fArray {
    fn slice<'a>(&'a self) -> &'a [f32] {
        let pointer = unsafe { bindings::cusd_Vec2fArray_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_Vec2fArray_size(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer, size * 2) }
    }
}

struct IntArray {
    inner: bindings::cusd_IntArray,
}

impl IntArray {
    fn slice<'a>(&'a self) -> &'a [i32] {
        let pointer = unsafe { bindings::cusd_IntArray_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_IntArray_size(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer, size) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
struct Attribute {
    inner: bindings::cusd_Attribute,
}

impl Attribute {
    fn get_vec3f_array(&self) -> Vec3fArray {
        Vec3fArray {
            inner: unsafe { bindings::cusd_Attribute_get_vec3f_array(&self.inner) },
        }
    }

    fn get_vec2f_array(&self) -> Vec2fArray {
        Vec2fArray {
            inner: unsafe { bindings::cusd_Attribute_get_vec2f_array(&self.inner) },
        }
    }

    fn get_int_array(&self) -> IntArray {
        IntArray {
            inner: unsafe { bindings::cusd_Attribute_get_int_array(&self.inner) },
        }
    }

    fn get_type_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(bindings::cusd_Attribute_get_type_name(&self.inner)) }
    }

    fn get_token_metadata(&self, name: &Token) -> Option<Token> {
        let mut token = Default::default();

        let valid = unsafe {
            bindings::cusd_Attribute_get_token_metadata(&self.inner, &name.inner, &mut token)
        };

        if valid {
            Some(Token{inner: token})
        } else {
            None
        }
    }
}

struct Prim {
    inner: bindings::cusd_Prim,
}

impl Prim {
    fn get_type_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(bindings::cusd_Prim_get_type_name(&self.inner)) }
    }

    fn get_attribute(&self, name: &Token) -> Option<Attribute> {
        let mut attribute = Default::default();

        let valid = unsafe {
            bindings::cusd_Prim_get_attribute(&self.inner, &name.inner, &mut attribute)
        };

        if valid {
            Some(Attribute { inner: attribute})
        } else {
            None
        }
    }

    fn compute_bound_material(&self) -> Material {
        Material {
            inner: unsafe { bindings::cusd_Prim_compute_bound_material(&self.inner) },
        }
    }

    fn get_all_subsets(&self) -> Subsets {
        Subsets {
            inner: unsafe { bindings::cusd_Prim_get_all_subsets(&self.inner) },
        }
    }
}

struct XformCache {
    inner: bindings::cusd_GeomXformCache,
}

impl XformCache {
    fn new() -> Self {
        Self {
            inner: unsafe { bindings::cusd_GeomXformCache_new() },
        }
    }

    fn get_transform(&mut self, prim: &Prim) -> [f64; 16] {
        let mut transform = [0.0f64; 16];
        unsafe {
            bindings::cusd_GeomXformCache_get_transform(
                &mut self.inner,
                &prim.inner,
                transform.as_mut_ptr(),
            )
        };
        transform
    }
}

struct Token {
    inner: bindings::cusd_Token,
}

impl Token {
    fn new(text: &[u8]) -> Self {
        Self {
            inner: unsafe {
                // todo: this deref is a hack.
                *bindings::cusd_Token_new(text.as_ptr() as *const _)
            }
        }
    }

    fn str<'a>(&'a self) -> &'a str {
        let pointer = unsafe { bindings::cusd_Token_pointer(&self.inner) };
        let size = unsafe { bindings::cusd_Token_size(&self.inner) };

        std::str::from_utf8(unsafe { std::slice::from_raw_parts(pointer as *const u8, size) })
            .unwrap()
    }

    fn equal(&self, other: &Self) -> bool {
        unsafe {
            bindings::cusd_Token_equal(&self.inner, &other.inner)
        }
    }
}
