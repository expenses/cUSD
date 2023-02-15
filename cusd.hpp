#include <pxr/usd/usd/attribute.h>
#include <pxr/usd/usd/prim.h>
#include <pxr/usd/usd/primRange.h>
#include <pxr/usd/usd/stage.h>
// We only need usdGeom for xformCache, which is something
// that could potentially be replaced by a Rust impl.
#include <pxr/usd/usdGeom/xformCache.h>
#include <pxr/usd/usdShade/materialBindingAPI.h>
#include <iostream>

// Wish C/C++ had hygenic macros :(
#define GENERATE_BINDING(NAME, INNER_TYPE)  \
    struct NAME {                           \
        INNER_TYPE inner;                   \
    };                                      \
                                            \
    void NAME##_free(const NAME& binding) { \
        binding.~NAME();                    \
    }

#define GENERATE_POINTER_AND_SIZE_FNS(NAME, ITEM_TYPE, SIZE_MULTIPLIER) \
    const ITEM_TYPE* NAME##_pointer(const NAME& binding) {              \
        return reinterpret_cast<const ITEM_TYPE *>(binding.inner.data());   \
    }                                                                       \
                                                                            \
    size_t NAME##_size(const NAME& binding) {                               \
        return binding.inner.size() * SIZE_MULTIPLIER;                      \
    }                                       

extern "C"
{
    GENERATE_BINDING(cusd_Stage, pxr::UsdStageRefPtr);

    GENERATE_BINDING(cusd_PrimRange, pxr::UsdPrimRange);

    GENERATE_BINDING(cusd_Prim, pxr::UsdPrim);

    GENERATE_BINDING(cusd_Attribute, pxr::UsdAttribute);

    GENERATE_BINDING(cusd_GeomXformCache, pxr::UsdGeomXformCache);

    GENERATE_BINDING(cusd_IntArray, pxr::VtArray<int>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_IntArray, int, 1);

    GENERATE_BINDING(cusd_Int64Array, pxr::VtArray<int64_t>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_Int64Array, int64_t, 1);

    GENERATE_BINDING(cusd_Vec3fArray, pxr::VtArray<pxr::GfVec3f>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec3fArray, float, 3);

    GENERATE_BINDING(cusd_Vec2fArray, pxr::VtArray<pxr::GfVec2f>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec2fArray, float, 2);

    GENERATE_BINDING(cusd_Vec3dArray, pxr::VtArray<pxr::GfVec3d>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec3dArray, double, 3);

    GENERATE_BINDING(cusd_Vec2dArray, pxr::VtArray<pxr::GfVec2d>);
    
    GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec2dArray, double, 2);

    GENERATE_BINDING(cusd_FloatArray, pxr::VtArray<float>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_FloatArray, float, 1);

    GENERATE_BINDING(cusd_ShadeConnectionSourceInfo, pxr::UsdShadeConnectionSourceInfo);

    // Note: internally this is a `TfSmallVector<UsdShadeConnectionSourceInfo, 1>`.
    GENERATE_BINDING(cusd_ShadeSourceInfoVector, pxr::UsdShadeSourceInfoVector);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_ShadeSourceInfoVector, cusd_ShadeConnectionSourceInfo, 1);

    GENERATE_BINDING(cusd_String, std::string);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_String, char, 1);

    GENERATE_BINDING(cusd_Token, pxr::TfToken);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_Token, char, 1);

    GENERATE_BINDING(cusd_Layer, pxr::SdfLayerHandle);

    GENERATE_BINDING(cusd_LayerVector, pxr::SdfLayerHandleVector);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_LayerVector, cusd_Layer, 1);

    GENERATE_BINDING(cusd_PrimSiblingRange, pxr::UsdPrimSiblingRange);

    GENERATE_BINDING(cusd_AttributeVector, std::vector<pxr::UsdAttribute>);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_AttributeVector, cusd_Attribute, 1);

    cusd_Stage *cusd_Stage_open(const char *filename) {
        // Allocate the stage on the heap.
        return new cusd_Stage{pxr::UsdStage::Open(filename)};
    }

    const cusd_PrimRange cusd_Stage_iter_prims(const cusd_Stage &stage) {
        return {pxr::UsdPrimRange::Stage(stage.inner)};
    }

    void cusd_Stage_reload(const cusd_Stage &stage) {
        return stage.inner->Reload();
    }

    cusd_LayerVector cusd_Stage_get_used_layers(const cusd_Stage &stage) {
        return {stage.inner->GetUsedLayers()};
    }

    cusd_String cusd_String_default() {
        return {std::string()};
    }

    bool cusd_Layer_get_path(const cusd_Layer &layer, cusd_String &out_string) {
        const pxr::ArResolvedPath &path = layer.inner->GetResolvedPath();

        if (!path) {
            return false;
        }

        out_string.inner = path.GetPathString();

        return true;
    }

    bool cusd_Layer_reload(const cusd_Layer &layer) {
        return layer.inner->Reload();
    }

    bool cusd_PrimRange_is_empty(const cusd_PrimRange &iterator) {
        return iterator.inner.empty();
    }

    cusd_Prim cusd_PrimRange_next(cusd_PrimRange &iterator) {
        cusd_Prim prim = {iterator.inner.front()};
        iterator.inner.increment_begin();
        return prim;
    }

    bool cusd_PrimSiblingRange_is_empty(const cusd_PrimSiblingRange &iterator) {
        return iterator.inner.empty();
    }

    cusd_Prim cusd_PrimSiblingRange_next(cusd_PrimSiblingRange &iterator) {
        cusd_Prim prim = {iterator.inner.front()};
        iterator.inner.advance_begin(1);
        return prim;
    }

    const char *cusd_Prim_get_type_name(const cusd_Prim &prim) {
        return prim.inner.GetTypeName().data();
    }

    const char *cusd_Attribute_get_type_name(const cusd_Attribute &attribute) {
        return attribute.inner.GetTypeName().GetCPPTypeName().data();
    }

    bool cusd_Prim_get_attribute(const cusd_Prim &prim, const cusd_Token &token, cusd_Attribute &out_attribute) {
        pxr::UsdAttribute attribute = prim.inner.GetAttribute(token.inner);
        if (!attribute) {
            return false;
        }

        out_attribute.inner = attribute;

        return true;
    }

    cusd_AttributeVector cusd_Prim_get_attributes(const cusd_Prim& prim) {
        return {prim.inner.GetAttributes()};
    }

    double cusd_Attribute_get_double(const cusd_Attribute &attribute) {
        double value;
        attribute.inner.Get(&value);
        return value;
    }

    cusd_GeomXformCache cusd_GeomXformCache_new() {
        return {};
    }

    void cusd_GeomXformCache_get_transform(cusd_GeomXformCache &cache, const cusd_Prim &prim, double *array) {
        pxr::GfMatrix4d transform = cache.inner.GetLocalToWorldTransform(prim.inner);
        memcpy(array, transform.data(), sizeof(double) * 16);
    }

    cusd_IntArray cusd_Attribute_get_int_array(const cusd_Attribute &attribute) {
        pxr::VtArray<int> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_Int64Array cusd_Attribute_get_int64_array(const cusd_Attribute &attribute) {
        pxr::VtArray<int64_t> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_Token* cusd_Attribute_get_token(const cusd_Attribute& attribute) {
        pxr::TfToken value;
        attribute.inner.Get(&value);
        return new cusd_Token {value};
    }

    bool cusd_Attribute_get_token_metadata(const cusd_Attribute &attribute, const cusd_Token &name,
                                           cusd_Token &out_token) {
        return attribute.inner.GetMetadata(name.inner, &out_token.inner);
    }

    cusd_Vec3fArray cusd_Attribute_get_vec3f_array(const cusd_Attribute &attribute) {
        pxr::VtArray<pxr::GfVec3f> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_Vec2fArray cusd_Attribute_get_vec2f_array(const cusd_Attribute &attribute) {
        pxr::VtArray<pxr::GfVec2f> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_Vec3dArray cusd_Attribute_get_vec3d_array(const cusd_Attribute &attribute) {
        pxr::VtArray<pxr::GfVec3d> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_Vec2dArray cusd_Attribute_get_vec2d_array(const cusd_Attribute &attribute) {
        pxr::VtArray<pxr::GfVec2d> value;
        attribute.inner.Get(&value);
        return {value};
    }

    cusd_FloatArray cusd_Attribute_get_float_array(const cusd_Attribute& attribute) {
        pxr::VtArray<float> value;
        attribute.inner.Get(&value);
        return {value};
    }

    bool cusd_Attribute_get_bool(const cusd_Attribute& attribute) {
        bool value;
        attribute.inner.Get(&value);
        return value;
    }

    float cusd_Attribute_get_float(const cusd_Attribute& attribute) {
        float value;
        attribute.inner.Get(&value);
        return value;
    }

    int cusd_Attribute_get_int(const cusd_Attribute& attribute) {
        int value;
        attribute.inner.Get(&value);
        return value;
    }

    void cusd_Attribute_get_string(const cusd_Attribute& attribute, cusd_String& output) {
        std::string output_tmp;
        attribute.inner.Get(&output_tmp);
        if (output_tmp.size() > 0) {
            output.inner = output_tmp;     
        }   
    }

    void cusd_Attribute_get_vec3f(const cusd_Attribute& attribute, float* output) {
        pxr::GfVec3f value;
        attribute.inner.Get(&value);
        memcpy(output, value.data(), sizeof(float) * 3);
    }

    void cusd_Attribute_get_vec2f(const cusd_Attribute& attribute, float* output) {
        pxr::GfVec2f value;
        attribute.inner.Get(&value);
        memcpy(output, value.data(), sizeof(float) * 2);
    }

    void cusd_Attribute_get_vec3d(const cusd_Attribute& attribute, double* output) {
        pxr::GfVec3d value;
        attribute.inner.Get(&value);
        memcpy(output, value.data(), sizeof(double) * 3);
    }

    void cusd_Attribute_get_vec2d(const cusd_Attribute& attribute, double* output) {
        pxr::GfVec2d value;
        attribute.inner.Get(&value);
        memcpy(output, value.data(), sizeof(double) * 2);
    }

    cusd_Prim cusd_Prim_compute_bound_material(const cusd_Prim &prim) {
        pxr::UsdShadeMaterialBindingAPI api = pxr::UsdShadeMaterialBindingAPI(prim.inner);
        pxr::UsdShadeMaterial material = api.ComputeBoundMaterial(pxr::UsdShadeTokens->full);
        return {material.GetPrim()};
    }

    bool cusd_Prim_compute_material_surface_source(const cusd_Prim &material, cusd_Prim &shader_prim) {
        pxr::UsdShadeShader source = pxr::UsdShadeMaterial(material.inner).ComputeSurfaceSource();
        if (!source) {
            return false;
        }
        shader_prim.inner = source.GetPrim();
        return true;
    }

    bool cusd_Attribute_get_resolved_path(const cusd_Attribute &attribute, cusd_String &out_path) {
        pxr::SdfAssetPath path;
        if (!attribute.inner.Get(&path)) {
            return false;
        }

        const std::string &resolved_path = path.GetResolvedPath();

        if (resolved_path.size() == 0) {
            return false;
        }

        out_path.inner = resolved_path;

        return true;
    }

    cusd_ShadeSourceInfoVector cusd_Attribute_get_connected_shade_sources(const cusd_Attribute &attribute) {
        return {pxr::UsdShadeConnectableAPI::GetConnectedSources(attribute.inner)};
    }

    cusd_PrimSiblingRange cusd_Prim_get_all_children(const cusd_Prim &prim) {
        return {prim.inner.GetAllChildren()};
    }

    cusd_Prim cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo &source_info) {
        return {source_info.inner.source.GetPrim()};
    }

    cusd_Token *cusd_Token_new(const char *text) {
        // todo: had problems without this `new`.
        return new cusd_Token{pxr::TfToken(text)};
    }

    bool cusd_Token_equal(const cusd_Token &a, const cusd_Token &b) {
        return a.inner == b.inner;
    }

    cusd_Token* cusd_Attribute_get_base_name(const cusd_Attribute& attribute) {
        return new cusd_Token{attribute.inner.GetBaseName()};
    }

    cusd_Token* cusd_Attribute_get_namespace(const cusd_Attribute& attribute) {
        return new cusd_Token{attribute.inner.GetNamespace()};
    }
}
