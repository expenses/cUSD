mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::sys::*;
use std::fmt;
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

pub struct VtArray<T> {
    inner: cusd_VtArray,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> VtArray<T> {
    pub fn new(array: cusd_VtArray) -> Self {
        let this = Self {
            inner: array,
            _phantom: Default::default(),
        };

        if !this.is_empty() {
            //dbg!(&this.slice()[this.slice().len().saturating_sub(1)]);
        }

        this
    }
}

impl<T> Deref for VtArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let size = unsafe { cusd_VtArray_size(&self.inner) };

        let pointer = unsafe { cusd_VtArray_pointer(&self.inner) };

        unsafe { std::slice::from_raw_parts(pointer as *const T, size) }
    }
}

impl<T> Drop for VtArray<T> {
    fn drop(&mut self) {
        unsafe {
            cusd_VtArray_free(&self.inner);
        }
    }
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

generate_binding!(Stage, cusd_Stage, cusd_Stage_free);

impl Stage {
    pub fn open(filename: &std::ffi::CStr) -> Self {
        Self {
            inner: unsafe { cusd_Stage_open(filename.as_ptr()) },
        }
    }

    pub fn iter_prims(&self) -> PrimRange {
        PrimRange {
            inner: unsafe { cusd_Stage_iter_prims(&self.inner) },
        }
    }

    pub fn get_used_layers(&self) -> LayerVector {
        LayerVector {
            inner: unsafe { cusd_Stage_get_used_layers(&self.inner) },
        }
    }

    pub fn reload(&self) {
        unsafe { cusd_Stage_reload(&self.inner) }
    }
}

generate_binding!(
    ShadeSourceInfo,
    cusd_ShadeConnectionSourceInfo,
    cusd_ShadeConnectionSourceInfo_free
);

impl ShadeSourceInfo {
    pub fn source(&self) -> Prim {
        Prim {
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
    PrimSublingRange,
    cusd_PrimSiblingRange,
    cusd_PrimSiblingRange_free
);

impl Iterator for PrimSublingRange {
    type Item = Prim;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if cusd_PrimSiblingRange_is_empty(&self.inner) {
                return None;
            }

            return Some(Prim {
                inner: cusd_PrimSiblingRange_next(&mut self.inner),
            });
        }
    }
}

generate_binding!(Attribute, cusd_Attribute, cusd_Attribute_free);

pub enum AttributeValue {
    Vec3fArray(VtArray<[f32; 3]>),
    Vec2fArray(VtArray<[f32; 2]>),
    Vec3dArray(VtArray<[f64; 3]>),
    Vec2dArray(VtArray<[f64; 2]>),
    TokenArray(VtArray<Token>),
    StringArray(VtArray<CusdString>),
    IntArray(VtArray<i32>),
    Int64Array(VtArray<i64>),
    Token(Token),
    FloatArray(VtArray<f32>),
    DoubleArray(VtArray<f64>),
    Bool(bool),
    Float(f32),
    Double(f64),
    Vec3f([f32; 3]),
    Vec2f([f32; 2]),
    Vec3d([f64; 3]),
    Vec2d([f64; 2]),
    Vec4f([f32; 4]),
    Quatf([f32; 4]),
    Int(i32),
    String(CusdString),
}

impl Attribute {
    pub fn has_authored_connections(&self) -> bool {
        unsafe { cusd_Attribute_has_authored_connections(&self.inner) }
    }

    pub fn has_authored_value(&self) -> bool {
        unsafe { cusd_Attribute_has_authored_value(&self.inner) }
    }

    pub fn has_value(&self) -> bool {
        unsafe { cusd_Attribute_has_value(&self.inner) }
    }

    pub fn is_authored(&self) -> bool {
        unsafe { cusd_Attribute_is_authored(&self.inner) }
    }

    pub fn get_value(&self) -> Option<AttributeValue> {
        //if self.get_type_name().to_bytes() == b"VtArray<float>" {
        //dbg!(self.has_value(), self.has_authored_value(), self.has_authored_connections(), self.is_authored());

        //dbg!(&*self.get_namespace(), &*self.get_base_name(), self.get_type_name());
        //}

        Some(match self.get_type_name().to_bytes() {
            b"VtArray<int>" => AttributeValue::IntArray(self.get_int_array()),
            b"VtArray<GfVec3f>" => AttributeValue::Vec3fArray(self.get_vec3f_array()?),
            b"VtArray<GfVec2f>" => AttributeValue::Vec2fArray(self.get_vec2f_array()?),
            b"VtArray<float>" => AttributeValue::FloatArray(self.get_float_array()?),
            b"VtArray<double>" => AttributeValue::DoubleArray(self.get_double_array()),
            b"VtArray<GfVec3d>" => AttributeValue::Vec3dArray(self.get_vec3d_array()?),
            b"VtArray<GfVec2d>" => AttributeValue::Vec2dArray(self.get_vec2d_array()?),
            b"VtArray<TfToken>" => AttributeValue::TokenArray(self.get_token_array()?),
            b"VtArray<std::string>" => AttributeValue::StringArray(self.get_string_array()),
            b"TfToken" => AttributeValue::Token(self.get_token()),
            b"bool" => AttributeValue::Bool(self.get_bool()),
            b"float" => AttributeValue::Float(self.get_float()),
            b"double" => AttributeValue::Double(self.get_double()?),
            b"GfVec4f" => AttributeValue::Vec4f(self.get_vec4f()?),
            b"GfQuatf" => AttributeValue::Vec4f(self.get_quatf()?),
            b"GfVec3f" => AttributeValue::Vec3f(self.get_vec3f()?),
            b"GfVec2f" => AttributeValue::Vec2f(self.get_vec2f()?),
            b"GfVec3d" => AttributeValue::Vec3d(self.get_vec3d()?),
            b"GfVec2d" => AttributeValue::Vec2d(self.get_vec2d()?),
            b"int" => AttributeValue::Int(self.get_int()),
            b"std::string" => AttributeValue::String(self.get_string()?),
            b"VtArray<int64_t>" => AttributeValue::Int64Array(self.get_int64_array()),
            other => {
                dbg!(std::str::from_utf8(other).unwrap());
                return None;
            }
        })
    }

    pub fn get_float(&self) -> f32 {
        unsafe { cusd_Attribute_get_float(&self.inner) }
    }

    pub fn get_double(&self) -> Option<f64> {
        get_checked(|value| unsafe { cusd_Attribute_get_double(&self.inner, value) })
    }

    pub fn get_bool(&self) -> bool {
        unsafe { cusd_Attribute_get_bool(&self.inner) }
    }

    pub fn get_int(&self) -> i32 {
        unsafe { cusd_Attribute_get_int(&self.inner) }
    }

    pub fn get_string(&self) -> Option<CusdString> {
        get_checked(|value| unsafe { cusd_Attribute_get_string(&self.inner, value) })
            .map(|inner| CusdString { inner })
    }

    pub fn get_vec3f(&self) -> Option<[f32; 3]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_vec3f(&self.inner, value) })
    }

    pub fn get_vec4f(&self) -> Option<[f32; 4]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_vec4f(&self.inner, value) })
    }

    pub fn get_quatf(&self) -> Option<[f32; 4]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_quatf(&self.inner, value) })
    }

    pub fn get_vec2f(&self) -> Option<[f32; 2]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_vec2f(&self.inner, value) })
    }

    pub fn get_vec3d(&self) -> Option<[f64; 3]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_vec3d(&self.inner, value) })
    }

    pub fn get_vec2d(&self) -> Option<[f64; 2]> {
        get_checked_array(|value| unsafe { cusd_Attribute_get_vec2d(&self.inner, value) })
    }

    pub fn get_vec3f_array(&self) -> Option<VtArray<[f32; 3]>> {
        get_checked(|value| unsafe { cusd_Attribute_get_vec3f_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_token_array(&self) -> Option<VtArray<Token>> {
        get_checked(|value| unsafe { cusd_Attribute_get_token_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_string_array(&self) -> VtArray<CusdString> {
        VtArray::new(unsafe { cusd_Attribute_get_string_array(&self.inner) })
    }

    pub fn get_vec2f_array(&self) -> Option<VtArray<[f32; 2]>> {
        get_checked(|value| unsafe { cusd_Attribute_get_vec2f_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_vec3d_array(&self) -> Option<VtArray<[f64; 3]>> {
        get_checked(|value| unsafe { cusd_Attribute_get_vec3d_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_vec2d_array(&self) -> Option<VtArray<[f64; 2]>> {
        get_checked(|value| unsafe { cusd_Attribute_get_vec2d_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_int_array(&self) -> VtArray<i32> {
        VtArray::new(unsafe { cusd_Attribute_get_int_array(&self.inner) })
    }

    pub fn get_int64_array(&self) -> VtArray<i64> {
        VtArray::new(unsafe { cusd_Attribute_get_int64_array(&self.inner) })
    }

    pub fn get_float_array(&self) -> Option<VtArray<f32>> {
        get_checked(|value| unsafe { cusd_Attribute_get_float_array(&self.inner, value) })
            .map(VtArray::new)
    }

    pub fn get_double_array(&self) -> VtArray<f64> {
        VtArray::new(unsafe { cusd_Attribute_get_double_array(&self.inner) })
    }

    pub fn get_token(&self) -> Token {
        Token {
            inner: unsafe { cusd_Attribute_get_token(&self.inner) },
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

    pub fn get_connected_shade_sources(&self) -> ShadeSourceInfoVector {
        ShadeSourceInfoVector {
            inner: unsafe { cusd_Attribute_get_connected_shade_sources(&self.inner) },
        }
    }

    pub fn get_resolved_path(&self) -> Option<CusdString> {
        let mut path = Default::default();

        let valid = unsafe { cusd_Attribute_get_resolved_path(&self.inner, &mut path) };

        if valid {
            Some(CusdString { inner: path })
        } else {
            None
        }
    }

    pub fn get_base_name(&self) -> Token {
        Token {
            inner: unsafe { cusd_Attribute_get_base_name(&self.inner) },
        }
    }

    pub fn get_namespace(&self) -> Token {
        Token {
            inner: unsafe { cusd_Attribute_get_namespace(&self.inner) },
        }
    }
}

fn get_checked_array<T: Default + Copy + std::fmt::Debug, const N: usize, F: Fn(&mut T) -> bool>(
    function: F,
) -> Option<[T; N]> {
    let mut array = [T::default(); N];

    let valid = function(&mut array[0]);

    if valid {
        Some(array)
    } else {
        None
    }
}

fn get_checked<T: Default, F: Fn(&mut T) -> bool>(function: F) -> Option<T> {
    let mut value = T::default();

    let valid = function(&mut value);

    if valid {
        Some(value)
    } else {
        None
    }
}

generate_binding!(
    AttributeVector,
    cusd_AttributeVector,
    cusd_AttributeVector_free
);
deref_into_slice!(
    AttributeVector,
    Attribute,
    cusd_AttributeVector_pointer,
    cusd_AttributeVector_size
);

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

    pub fn compute_bound_material(&self) -> Prim {
        Prim {
            inner: unsafe { cusd_Prim_compute_bound_material(&self.inner) },
        }
    }

    pub fn get_all_children(&self) -> PrimSublingRange {
        PrimSublingRange {
            inner: unsafe { cusd_Prim_get_all_children(&self.inner) },
        }
    }

    pub fn compute_material_surface_shader(&self) -> Option<Prim> {
        let mut shader = Default::default();

        let valid = unsafe { cusd_Prim_compute_material_surface_source(&self.inner, &mut shader) };

        if valid {
            Some(Prim { inner: shader })
        } else {
            None
        }
    }

    pub fn get_attributes(&self) -> AttributeVector {
        AttributeVector {
            inner: unsafe { cusd_Prim_get_attributes(&self.inner) },
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
    pub fn new(text: &'static [u8]) -> Self {
        Self {
            inner: unsafe {
                // todo: this deref is a hack.
                cusd_Token_new(text.as_ptr() as *const _)
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.deref().fmt(f)
    }
}
