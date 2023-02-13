mod bindings;
use std::mem::ManuallyDrop;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let c_filename = std::ffi::CString::new(&*filename).unwrap();

    dbg!(&filename, &c_filename);

    let ptr = c_filename.as_ptr();

    let stage = unsafe { bindings::cusd_Stage_open(ptr) };

    let mut cache = XformCache::new();

    let mut iterator = PrimIterator {
        inner: unsafe { bindings::cusd_Stage_iter_prims(stage) },
    };

    for prim in iterator {
        let type_name = prim.get_type_name();

        if type_name.to_bytes() == b"Sphere" {
            let display_colours = prim
                .get_attribute(b"primvars:displayColor\0")
                .get_vec3f_array();
            //dbg!(&display_colours.slice());
        } else if type_name.to_bytes() == b"Mesh" {
            for prim in prim.get_all_subsets().slice() {
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
                    dbg!(&string.str());
                }
            }

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
                    dbg!(&string.str());
                }
            //let prim = diffuse_input.get_prim();
            //dbg!(&prim.get_type_name());

            /*let vertex_counts = prim.get_attribute(b"faceVertexCounts\0").get_int_array();
            let vertex_indices = prim.get_attribute(b"faceVertexIndices\0").get_int_array();
            dbg!(vertex_counts.slice().len(), vertex_indices.slice().len());



            for subset in prim.get_all_subsets().slice() {
                //dbg!(subset.get_indices_attr().get_int_array().slice().len());
            }*/
        } else {
            //dbg!(&type_name);
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
    fn get_input(&self, name: &[u8]) -> ShadeInput {
        ShadeInput {
            inner: unsafe {
                bindings::cusd_ShadeShader_get_input(&self.inner, name.as_ptr() as *const i8)
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

struct Attribute {
    inner: bindings::cusd_Attribute,
}

impl Attribute {
    fn get_vec3f_array(&self) -> Vec3fArray {
        Vec3fArray {
            inner: unsafe { bindings::cusd_Attribute_get_vec3f_array(&self.inner) },
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
}

struct Prim {
    inner: bindings::cusd_Prim,
}

impl Prim {
    fn get_type_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(bindings::cusd_Prim_get_type_name(&self.inner)) }
    }

    fn get_attribute(&self, name: &[u8]) -> Attribute {
        Attribute {
            inner: unsafe {
                bindings::cusd_Prim_get_attribute(&self.inner, name.as_ptr() as *const i8)
            },
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
