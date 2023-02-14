/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Stage {
    pub _bindgen_opaque_blob: u64,
}
#[test]
fn bindgen_test_layout_cusd_Stage() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Stage>(),
        8usize,
        concat!("Size of: ", stringify!(cusd_Stage))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Stage>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Stage))
    );
}
extern "C" {
    pub fn cusd_Stage_free(binding: *const cusd_Stage);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_PrimRange {
    pub _bindgen_opaque_blob: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_cusd_PrimRange() {
    assert_eq!(
        ::std::mem::size_of::<cusd_PrimRange>(),
        56usize,
        concat!("Size of: ", stringify!(cusd_PrimRange))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_PrimRange>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_PrimRange))
    );
}
extern "C" {
    pub fn cusd_PrimRange_free(binding: *const cusd_PrimRange);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Prim {
    pub _bindgen_opaque_blob: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_cusd_Prim() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Prim>(),
        32usize,
        concat!("Size of: ", stringify!(cusd_Prim))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Prim>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Prim))
    );
}
extern "C" {
    pub fn cusd_Prim_free(binding: *const cusd_Prim);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Attribute {
    pub _bindgen_opaque_blob: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_cusd_Attribute() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Attribute>(),
        32usize,
        concat!("Size of: ", stringify!(cusd_Attribute))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Attribute>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Attribute))
    );
}
extern "C" {
    pub fn cusd_Attribute_free(binding: *const cusd_Attribute);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_GeomXformCache {
    pub _bindgen_opaque_blob: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_cusd_GeomXformCache() {
    assert_eq!(
        ::std::mem::size_of::<cusd_GeomXformCache>(),
        64usize,
        concat!("Size of: ", stringify!(cusd_GeomXformCache))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_GeomXformCache>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_GeomXformCache))
    );
}
extern "C" {
    pub fn cusd_GeomXformCache_free(binding: *const cusd_GeomXformCache);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_IntArray {
    pub _bindgen_opaque_blob: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_cusd_IntArray() {
    assert_eq!(
        ::std::mem::size_of::<cusd_IntArray>(),
        40usize,
        concat!("Size of: ", stringify!(cusd_IntArray))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_IntArray>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_IntArray))
    );
}
extern "C" {
    pub fn cusd_IntArray_free(binding: *const cusd_IntArray);
}
extern "C" {
    pub fn cusd_IntArray_pointer(binding: *const cusd_IntArray) -> *const ::std::os::raw::c_int;
}
extern "C" {
    pub fn cusd_IntArray_size(binding: *const cusd_IntArray) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Vec3fArray {
    pub _bindgen_opaque_blob: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_cusd_Vec3fArray() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Vec3fArray>(),
        40usize,
        concat!("Size of: ", stringify!(cusd_Vec3fArray))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Vec3fArray>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Vec3fArray))
    );
}
extern "C" {
    pub fn cusd_Vec3fArray_free(binding: *const cusd_Vec3fArray);
}
extern "C" {
    pub fn cusd_Vec3fArray_pointer(binding: *const cusd_Vec3fArray) -> *const f32;
}
extern "C" {
    pub fn cusd_Vec3fArray_size(binding: *const cusd_Vec3fArray) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Vec2fArray {
    pub _bindgen_opaque_blob: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_cusd_Vec2fArray() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Vec2fArray>(),
        40usize,
        concat!("Size of: ", stringify!(cusd_Vec2fArray))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Vec2fArray>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Vec2fArray))
    );
}
extern "C" {
    pub fn cusd_Vec2fArray_free(binding: *const cusd_Vec2fArray);
}
extern "C" {
    pub fn cusd_Vec2fArray_pointer(binding: *const cusd_Vec2fArray) -> *const f32;
}
extern "C" {
    pub fn cusd_Vec2fArray_size(binding: *const cusd_Vec2fArray) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_ShadeMaterial {
    pub _bindgen_opaque_blob: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_cusd_ShadeMaterial() {
    assert_eq!(
        ::std::mem::size_of::<cusd_ShadeMaterial>(),
        24usize,
        concat!("Size of: ", stringify!(cusd_ShadeMaterial))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_ShadeMaterial>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_ShadeMaterial))
    );
}
extern "C" {
    pub fn cusd_ShadeMaterial_free(binding: *const cusd_ShadeMaterial);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_GeomSubset {
    pub _bindgen_opaque_blob: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_cusd_GeomSubset() {
    assert_eq!(
        ::std::mem::size_of::<cusd_GeomSubset>(),
        24usize,
        concat!("Size of: ", stringify!(cusd_GeomSubset))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_GeomSubset>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_GeomSubset))
    );
}
extern "C" {
    pub fn cusd_GeomSubset_free(binding: *const cusd_GeomSubset);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_GeomSubsetVector {
    pub _bindgen_opaque_blob: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_cusd_GeomSubsetVector() {
    assert_eq!(
        ::std::mem::size_of::<cusd_GeomSubsetVector>(),
        24usize,
        concat!("Size of: ", stringify!(cusd_GeomSubsetVector))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_GeomSubsetVector>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_GeomSubsetVector))
    );
}
extern "C" {
    pub fn cusd_GeomSubsetVector_free(binding: *const cusd_GeomSubsetVector);
}
extern "C" {
    pub fn cusd_GeomSubsetVector_pointer(
        binding: *const cusd_GeomSubsetVector,
    ) -> *const cusd_GeomSubset;
}
extern "C" {
    pub fn cusd_GeomSubsetVector_size(binding: *const cusd_GeomSubsetVector) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_ShadeShader {
    pub _bindgen_opaque_blob: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_cusd_ShadeShader() {
    assert_eq!(
        ::std::mem::size_of::<cusd_ShadeShader>(),
        24usize,
        concat!("Size of: ", stringify!(cusd_ShadeShader))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_ShadeShader>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_ShadeShader))
    );
}
extern "C" {
    pub fn cusd_ShadeShader_free(binding: *const cusd_ShadeShader);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_ShadeInput {
    pub _bindgen_opaque_blob: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_cusd_ShadeInput() {
    assert_eq!(
        ::std::mem::size_of::<cusd_ShadeInput>(),
        32usize,
        concat!("Size of: ", stringify!(cusd_ShadeInput))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_ShadeInput>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_ShadeInput))
    );
}
extern "C" {
    pub fn cusd_ShadeInput_free(binding: *const cusd_ShadeInput);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_ShadeConnectionSourceInfo {
    pub _bindgen_opaque_blob: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_cusd_ShadeConnectionSourceInfo() {
    assert_eq!(
        ::std::mem::size_of::<cusd_ShadeConnectionSourceInfo>(),
        56usize,
        concat!("Size of: ", stringify!(cusd_ShadeConnectionSourceInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_ShadeConnectionSourceInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_ShadeConnectionSourceInfo))
    );
}
extern "C" {
    pub fn cusd_ShadeConnectionSourceInfo_free(binding: *const cusd_ShadeConnectionSourceInfo);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_ShadeSourceInfoVector {
    pub _bindgen_opaque_blob: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_cusd_ShadeSourceInfoVector() {
    assert_eq!(
        ::std::mem::size_of::<cusd_ShadeSourceInfoVector>(),
        64usize,
        concat!("Size of: ", stringify!(cusd_ShadeSourceInfoVector))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_ShadeSourceInfoVector>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_ShadeSourceInfoVector))
    );
}
extern "C" {
    pub fn cusd_ShadeSourceInfoVector_free(binding: *const cusd_ShadeSourceInfoVector);
}
extern "C" {
    pub fn cusd_ShadeSourceInfoVector_pointer(
        binding: *const cusd_ShadeSourceInfoVector,
    ) -> *const cusd_ShadeConnectionSourceInfo;
}
extern "C" {
    pub fn cusd_ShadeSourceInfoVector_size(binding: *const cusd_ShadeSourceInfoVector) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_String {
    pub _bindgen_opaque_blob: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_cusd_String() {
    assert_eq!(
        ::std::mem::size_of::<cusd_String>(),
        32usize,
        concat!("Size of: ", stringify!(cusd_String))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_String>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_String))
    );
}
extern "C" {
    pub fn cusd_String_free(binding: *const cusd_String);
}
extern "C" {
    pub fn cusd_String_pointer(binding: *const cusd_String) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cusd_String_size(binding: *const cusd_String) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Token {
    pub _bindgen_opaque_blob: u64,
}
#[test]
fn bindgen_test_layout_cusd_Token() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Token>(),
        8usize,
        concat!("Size of: ", stringify!(cusd_Token))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Token>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Token))
    );
}
extern "C" {
    pub fn cusd_Token_free(binding: *const cusd_Token);
}
extern "C" {
    pub fn cusd_Token_pointer(binding: *const cusd_Token) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cusd_Token_size(binding: *const cusd_Token) -> usize;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_Layer {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_cusd_Layer() {
    assert_eq!(
        ::std::mem::size_of::<cusd_Layer>(),
        16usize,
        concat!("Size of: ", stringify!(cusd_Layer))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_Layer>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_Layer))
    );
}
extern "C" {
    pub fn cusd_Layer_free(binding: *const cusd_Layer);
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct cusd_LayerVector {
    pub _bindgen_opaque_blob: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_cusd_LayerVector() {
    assert_eq!(
        ::std::mem::size_of::<cusd_LayerVector>(),
        24usize,
        concat!("Size of: ", stringify!(cusd_LayerVector))
    );
    assert_eq!(
        ::std::mem::align_of::<cusd_LayerVector>(),
        8usize,
        concat!("Alignment of ", stringify!(cusd_LayerVector))
    );
}
extern "C" {
    pub fn cusd_LayerVector_free(binding: *const cusd_LayerVector);
}
extern "C" {
    pub fn cusd_LayerVector_pointer(binding: *const cusd_LayerVector) -> *const cusd_Layer;
}
extern "C" {
    pub fn cusd_LayerVector_size(binding: *const cusd_LayerVector) -> usize;
}
extern "C" {
    pub fn cusd_Stage_open(filename: *const ::std::os::raw::c_char) -> *mut cusd_Stage;
}
extern "C" {
    pub fn cusd_Stage_iter_prims(stage: *const cusd_Stage) -> cusd_PrimRange;
}
extern "C" {
    pub fn cusd_Stage_reload(stage: *const cusd_Stage);
}
extern "C" {
    pub fn cusd_Stage_get_used_layers(stage: *const cusd_Stage) -> cusd_LayerVector;
}
extern "C" {
    pub fn cusd_Layer_get_path(layer: *const cusd_Layer, out_string: *mut cusd_String) -> bool;
}
extern "C" {
    pub fn cusd_Layer_reload(layer: *const cusd_Layer) -> bool;
}
extern "C" {
    pub fn cusd_PrimRange_is_empty(iterator: *const cusd_PrimRange) -> bool;
}
extern "C" {
    pub fn cusd_PrimRange_next(iterator: *mut cusd_PrimRange) -> cusd_Prim;
}
extern "C" {
    pub fn cusd_Prim_get_type_name(prim: *const cusd_Prim) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cusd_Attribute_get_type_name(
        attribute: *const cusd_Attribute,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cusd_Prim_get_attribute(
        prim: *const cusd_Prim,
        token: *const cusd_Token,
        out_attribute: *mut cusd_Attribute,
    ) -> bool;
}
extern "C" {
    pub fn cusd_Attribute_get_double(attribute: *const cusd_Attribute) -> f64;
}
extern "C" {
    pub fn cusd_GeomXformCache_new() -> cusd_GeomXformCache;
}
extern "C" {
    pub fn cusd_GeomXformCache_get_transform(
        cache: *mut cusd_GeomXformCache,
        prim: *const cusd_Prim,
        array: *mut f64,
    );
}
extern "C" {
    pub fn cusd_Attribute_get_int_array(attribute: *const cusd_Attribute) -> cusd_IntArray;
}
extern "C" {
    pub fn cusd_Attribute_get_token_metadata(
        attribute: *const cusd_Attribute,
        name: *const cusd_Token,
        out_token: *mut cusd_Token,
    ) -> bool;
}
extern "C" {
    pub fn cusd_Attribute_get_vec3f_array(attribute: *const cusd_Attribute) -> cusd_Vec3fArray;
}
extern "C" {
    pub fn cusd_Attribute_get_vec2f_array(attribute: *const cusd_Attribute) -> cusd_Vec2fArray;
}
extern "C" {
    pub fn cusd_Prim_compute_bound_material(prim: *const cusd_Prim) -> cusd_ShadeMaterial;
}
extern "C" {
    pub fn cusd_GeomSubset_compute_bound_material(
        subset: *const cusd_GeomSubset,
    ) -> cusd_ShadeMaterial;
}
extern "C" {
    pub fn cusd_ShadeMaterial_compute_surface_source(
        material: *const cusd_ShadeMaterial,
        shader: *mut cusd_ShadeShader,
    ) -> bool;
}
extern "C" {
    pub fn cusd_ShadeShader_get_input(
        shader: *const cusd_ShadeShader,
        name: *const cusd_Token,
        out_input: *mut cusd_ShadeInput,
    ) -> bool;
}
extern "C" {
    pub fn cusd_ShadeInput_get_resolved_path(
        input: *const cusd_ShadeInput,
        out_path: *mut cusd_String,
    ) -> bool;
}
extern "C" {
    pub fn cusd_ShadeInput_get_connected_sources(
        input: *const cusd_ShadeInput,
    ) -> cusd_ShadeSourceInfoVector;
}
extern "C" {
    pub fn cusd_Prim_get_all_subsets(prim: *const cusd_Prim) -> cusd_GeomSubsetVector;
}
extern "C" {
    pub fn cusd_GeomSubset_get_indices_attr(subset: *const cusd_GeomSubset) -> cusd_Attribute;
}
extern "C" {
    pub fn cusd_ShadeConnectionSourceInfo_source(
        source_info: *const cusd_ShadeConnectionSourceInfo,
    ) -> cusd_ShadeShader;
}
extern "C" {
    pub fn cusd_Token_new(text: *const ::std::os::raw::c_char) -> *mut cusd_Token;
}
extern "C" {
    pub fn cusd_Token_equal(a: *const cusd_Token, b: *const cusd_Token) -> bool;
}
