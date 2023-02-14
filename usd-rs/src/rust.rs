use crate::bindings::*;
use std::ops::Deref;

// This macro reduces tha amount of boilerplate code and enforces
// that all structs are `repr(transparent)` (for casting reasons)
// and have a `Drop` impl.
macro_rules! generate_binding {
    ($name:ident, $inner:ty, $free_fn:expr) => {
        #[repr(transparent)]
        pub struct $name {
            inner: $inner,
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { $free_fn(&self.inner) }
            }
        }
    };
}

macro_rules! deref_into_slice {
    ($name:ty, $item:ty, $pointer_fn:expr, $size_fn:expr) => {
        impl Deref for $name {
            type Target = [$item];

            fn deref(&self) -> &Self::Target {
                let pointer = unsafe { $pointer_fn(&self.inner) };

                let size = unsafe { $size_fn(&self.inner) };

                unsafe { std::slice::from_raw_parts(pointer as *const $item, size) }
            }
        }
    };
}

generate_binding!(Layer, cusd_Layer, cusd_Layer_free);

impl Layer {
    pub fn reload(&self) -> bool {
        unsafe { cusd_Layer_reload(&self.inner) }
    }

    pub fn get_path(&self) -> Option<CusdString> {
        let mut path = Default::default();

        let valid = unsafe { cusd_Layer_get_path(&self.inner, &mut path) };

        if valid {
            Some(CusdString { inner: path })
        } else {
            None
        }
    }
}

generate_binding!(LayerVector, cusd_LayerVector, cusd_LayerVector_free);
deref_into_slice!(
    LayerVector,
    Layer,
    cusd_LayerVector_pointer,
    cusd_LayerVector_size
);

pub struct Stage {
    inner: *mut cusd_Stage,
}

impl Stage {
    pub fn open(filename: &std::ffi::CStr) -> Self {
        Self {
            inner: unsafe { cusd_Stage_open(filename.as_ptr()) },
        }
    }

    pub fn iter_prims(&self) -> PrimRange {
        PrimRange {
            inner: unsafe { cusd_Stage_iter_prims(self.inner) },
        }
    }

    pub fn get_used_layers(&self) -> LayerVector {
        LayerVector {
            inner: unsafe { cusd_Stage_get_used_layers(self.inner) },
        }
    }

    pub fn reload(&self) {
        unsafe { cusd_Stage_reload(self.inner) }
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        unsafe { cusd_Stage_free(self.inner) }
    }
}

generate_binding!(
    ShadeSourceInfo,
    cusd_ShadeConnectionSourceInfo,
    cusd_ShadeConnectionSourceInfo_free
);

impl ShadeSourceInfo {
    pub fn source(&self) -> ShadeShader {
        ShadeShader {
            inner: unsafe { cusd_ShadeConnectionSourceInfo_source(&self.inner) },
        }
    }
}

generate_binding!(
    ShadeSourceInfoVector,
    cusd_ShadeSourceInfoVector,
    cusd_ShadeSourceInfoVector_free
);
deref_into_slice!(
    ShadeSourceInfoVector,
    ShadeSourceInfo,
    cusd_ShadeSourceInfoVector_pointer,
    cusd_ShadeSourceInfoVector_size
);

generate_binding!(CusdString, cusd_String, cusd_String_free);

impl Deref for CusdString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let pointer = unsafe { cusd_String_pointer(&self.inner) };
        let size = unsafe { cusd_String_size(&self.inner) };

        std::str::from_utf8(unsafe { std::slice::from_raw_parts(pointer as *const u8, size) })
            .unwrap()
    }
}

generate_binding!(ShadeInput, cusd_ShadeInput, cusd_ShadeInput_free);

impl ShadeInput {
    pub fn get_all_connected_sources(&self) -> ShadeSourceInfoVector {
        ShadeSourceInfoVector {
            inner: unsafe { cusd_ShadeInput_get_connected_sources(&self.inner) },
        }
    }

    pub fn get_resolved_path(&self) -> Option<CusdString> {
        let mut path = Default::default();

        let valid = unsafe { cusd_ShadeInput_get_resolved_path(&self.inner, &mut path) };

        if valid {
            Some(CusdString { inner: path })
        } else {
            None
        }
    }
}

generate_binding!(ShadeShader, cusd_ShadeShader, cusd_ShadeShader_free);

impl ShadeShader {
    pub fn get_input(&self, name: &Token) -> Option<ShadeInput> {
        let mut input = Default::default();

        let valid = unsafe { cusd_ShadeShader_get_input(&self.inner, &name.inner, &mut input) };

        if valid {
            Some(ShadeInput { inner: input })
        } else {
            None
        }
    }
}

generate_binding!(ShadeMaterial, cusd_ShadeMaterial, cusd_ShadeMaterial_free);

impl ShadeMaterial {
    pub fn compute_surface_shader(&self) -> Option<ShadeShader> {
        let mut shader = Default::default();

        let valid = unsafe { cusd_ShadeMaterial_compute_surface_source(&self.inner, &mut shader) };

        if valid {
            Some(ShadeShader { inner: shader })
        } else {
            None
        }
    }
}

generate_binding!(GeomSubset, cusd_GeomSubset, cusd_GeomSubset_free);

impl GeomSubset {
    pub fn get_indices_attr(&self) -> Attribute {
        Attribute {
            inner: unsafe { cusd_GeomSubset_get_indices_attr(&self.inner) },
        }
    }

    pub fn compute_bound_material(&self) -> ShadeMaterial {
        ShadeMaterial {
            inner: unsafe { cusd_GeomSubset_compute_bound_material(&self.inner) },
        }
    }
}

generate_binding!(PrimRange, cusd_PrimRange, cusd_PrimRange_free);

impl Iterator for PrimRange {
    type Item = Prim;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if cusd_PrimRange_is_empty(&self.inner) {
                return None;
            }

            return Some(Prim {
                inner: cusd_PrimRange_next(&mut self.inner),
            });
        }
    }
}

generate_binding!(
    GeomSubsetVector,
    cusd_GeomSubsetVector,
    cusd_GeomSubsetVector_free
);
deref_into_slice!(
    GeomSubsetVector,
    GeomSubset,
    cusd_GeomSubsetVector_pointer,
    cusd_GeomSubsetVector_size
);

generate_binding!(Vec3fArray, cusd_Vec3fArray, cusd_Vec3fArray_free);
deref_into_slice!(
    Vec3fArray,
    f32,
    cusd_Vec3fArray_pointer,
    cusd_Vec3fArray_size
);

generate_binding!(Vec2fArray, cusd_Vec2fArray, cusd_Vec2fArray_free);
deref_into_slice!(
    Vec2fArray,
    f32,
    cusd_Vec2fArray_pointer,
    cusd_Vec2fArray_size
);

generate_binding!(IntArray, cusd_IntArray, cusd_IntArray_free);
deref_into_slice!(IntArray, i32, cusd_IntArray_pointer, cusd_IntArray_size);

generate_binding!(Attribute, cusd_Attribute, cusd_Attribute_free);

impl Attribute {
    pub fn get_vec3f_array(&self) -> Vec3fArray {
        Vec3fArray {
            inner: unsafe { cusd_Attribute_get_vec3f_array(&self.inner) },
        }
    }

    pub fn get_vec2f_array(&self) -> Vec2fArray {
        Vec2fArray {
            inner: unsafe { cusd_Attribute_get_vec2f_array(&self.inner) },
        }
    }

    pub fn get_int_array(&self) -> IntArray {
        IntArray {
            inner: unsafe { cusd_Attribute_get_int_array(&self.inner) },
        }
    }

    pub fn get_type_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(cusd_Attribute_get_type_name(&self.inner)) }
    }

    pub fn get_token_metadata(&self, name: &Token) -> Option<Token> {
        let mut token = Default::default();

        let valid =
            unsafe { cusd_Attribute_get_token_metadata(&self.inner, &name.inner, &mut token) };

        if valid {
            Some(Token { inner: token })
        } else {
            None
        }
    }
}

generate_binding!(Prim, cusd_Prim, cusd_Prim_free);

impl Prim {
    pub fn get_type_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(cusd_Prim_get_type_name(&self.inner)) }
    }

    pub fn get_attribute(&self, name: &Token) -> Option<Attribute> {
        let mut attribute = Default::default();

        let valid = unsafe { cusd_Prim_get_attribute(&self.inner, &name.inner, &mut attribute) };

        if valid {
            Some(Attribute { inner: attribute })
        } else {
            None
        }
    }

    pub fn compute_bound_material(&self) -> ShadeMaterial {
        ShadeMaterial {
            inner: unsafe { cusd_Prim_compute_bound_material(&self.inner) },
        }
    }

    pub fn get_all_subsets(&self) -> GeomSubsetVector {
        GeomSubsetVector {
            inner: unsafe { cusd_Prim_get_all_subsets(&self.inner) },
        }
    }
}

generate_binding!(
    GeomXformCache,
    cusd_GeomXformCache,
    cusd_GeomXformCache_free
);

impl GeomXformCache {
    pub fn new() -> Self {
        Self {
            inner: unsafe { cusd_GeomXformCache_new() },
        }
    }

    pub fn get_transform(&mut self, prim: &Prim) -> [f64; 16] {
        let mut transform = [0.0f64; 16];
        unsafe {
            cusd_GeomXformCache_get_transform(&mut self.inner, &prim.inner, transform.as_mut_ptr())
        };
        transform
    }
}

generate_binding!(Token, cusd_Token, cusd_Token_free);

impl Token {
    pub fn new(text: &[u8]) -> Self {
        Self {
            inner: unsafe {
                // todo: this deref is a hack.
                *cusd_Token_new(text.as_ptr() as *const _)
            },
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        unsafe { cusd_Token_equal(&self.inner, &other.inner) }
    }
}

impl Deref for Token {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let pointer = unsafe { cusd_Token_pointer(&self.inner) };
        let size = unsafe { cusd_Token_size(&self.inner) };

        std::str::from_utf8(unsafe { std::slice::from_raw_parts(pointer as *const u8, size) })
            .unwrap()
    }
}
