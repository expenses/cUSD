#include "cusd.h"

#include <pxr/usd/usd/attribute.h>
#include <pxr/usd/usd/prim.h>
#include <pxr/usd/usd/primRange.h>
#include <pxr/usd/usd/stage.h>
// We only need usdGeom for xformCache, which is something
// that could potentially be replaced by a Rust impl.
#include <iostream>
#include <pxr/usd/usdGeom/xformCache.h>
#include <pxr/usd/usdShade/materialBindingAPI.h>

// Wish C/C++ had hygenic macros :(
#define GENERATE_FREE(NAME, INNER_TYPE)                                                                                \
    \                                        
    void NAME##_free(const NAME *binding) {                                                                            \
        INNER_TYPE transmuted = transmute<INNER_TYPE>(*binding);                                                       \
    }

#define GENERATE_POINTER_AND_SIZE_FNS(NAME, INNER_TYPE, ITEM_TYPE)                                                     \
    const ITEM_TYPE *NAME##_pointer(const NAME *binding) {                                                             \
        return reinterpret_cast<const ITEM_TYPE *>(reinterpret_cast<const INNER_TYPE *>(binding)->data());             \
    }                                                                                                                  \
                                                                                                                       \
    size_t NAME##_size(const NAME *binding) { return reinterpret_cast<const INNER_TYPE *>(binding)->size(); }

template <class To, class From>
std::enable_if_t<sizeof(To) == sizeof(From) && alignof(To) == alignof(From), To> transmute(From from) {
    union Union {
        From from;
        To to;

        ~Union() {
        }
    };

    Union un = {};
    un.from = std::move(from);
    return std::move(un.to);
}

extern "C"
{
    GENERATE_FREE(cusd_VtArray, pxr::VtArray<uint8_t>);
    GENERATE_FREE(cusd_Token, pxr::TfToken);
    GENERATE_FREE(cusd_PrimRange, pxr::UsdPrimRange);
    GENERATE_FREE(cusd_Prim, pxr::UsdPrim);
    GENERATE_FREE(cusd_Attribute, pxr::UsdAttribute);
    GENERATE_FREE(cusd_GeomXformCache, pxr::UsdGeomXformCache);
    GENERATE_FREE(cusd_ShadeConnectionSourceInfo, pxr::UsdShadeConnectionSourceInfo);
    GENERATE_FREE(cusd_ShadeSourceInfoVector, pxr::UsdShadeSourceInfoVector);
    GENERATE_FREE(cusd_String, std::string);
    GENERATE_FREE(cusd_Layer, pxr::SdfLayerHandle);
    GENERATE_FREE(cusd_LayerVector, pxr::SdfLayerHandleVector);
    GENERATE_FREE(cusd_PrimSiblingRange, pxr::UsdPrimSiblingRange);
    GENERATE_FREE(cusd_AttributeVector, std::vector<pxr::UsdAttribute>);
    GENERATE_FREE(cusd_Stage, pxr::UsdStageRefPtr);

    GENERATE_POINTER_AND_SIZE_FNS(cusd_VtArray, pxr::VtArray<uint8_t>, uint8_t);
    GENERATE_POINTER_AND_SIZE_FNS(cusd_ShadeSourceInfoVector, pxr::UsdShadeSourceInfoVector,
                                  cusd_ShadeConnectionSourceInfo);
    GENERATE_POINTER_AND_SIZE_FNS(cusd_String, std::string, char);
    GENERATE_POINTER_AND_SIZE_FNS(cusd_LayerVector, pxr::SdfLayerHandleVector, cusd_Layer);
    GENERATE_POINTER_AND_SIZE_FNS(cusd_AttributeVector, std::vector<pxr::UsdAttribute>, cusd_Attribute);
    GENERATE_POINTER_AND_SIZE_FNS(cusd_Token, pxr::TfToken, char);

    cusd_Stage cusd_Stage_open(const char *filename) {
        return transmute<cusd_Stage>(pxr::UsdStage::Open(filename));
    }

    const cusd_PrimRange cusd_Stage_iter_prims(const cusd_Stage *stage) {
        auto stage_ptr = *reinterpret_cast<const pxr::UsdStageRefPtr *>(stage);
        return transmute<cusd_PrimRange>(pxr::UsdPrimRange::Stage(stage_ptr));
    }

    void cusd_Stage_reload(const cusd_Stage *stage) {
        auto stage_ptr = *reinterpret_cast<const pxr::UsdStageRefPtr *>(stage);
        return stage_ptr->Reload();
    }

    cusd_LayerVector cusd_Stage_get_used_layers(const cusd_Stage *stage) {
        auto stage_ptr = *reinterpret_cast<const pxr::UsdStageRefPtr *>(stage);
        return transmute<cusd_LayerVector>(stage_ptr->GetUsedLayers());
    }

    bool cusd_Layer_get_path(const cusd_Layer *layer, cusd_String *out_string) {
        auto layer_ptr = *reinterpret_cast<const pxr::SdfLayerHandle *>(layer);
        const pxr::ArResolvedPath &path = layer_ptr->GetResolvedPath();

        if (!path) {
            return false;
        }

        *reinterpret_cast<std::string *>(out_string) = path.GetPathString();

        return true;
    }

    bool cusd_Layer_reload(const cusd_Layer *layer) {
        auto layer_ptr = *reinterpret_cast<const pxr::SdfLayerHandle *>(layer);
        return layer_ptr->Reload();
    }

    bool cusd_PrimRange_is_empty(const cusd_PrimRange *iterator) {
        return reinterpret_cast<const pxr::UsdPrimRange *>(iterator)->empty();
    }

    cusd_Prim cusd_PrimRange_next(cusd_PrimRange *iterator) {
        cusd_Prim prim = transmute<cusd_Prim>(reinterpret_cast<pxr::UsdPrimRange *>(iterator)->front());
        reinterpret_cast<pxr::UsdPrimRange *>(iterator)->increment_begin();
        return prim;
    }

    bool cusd_PrimSiblingRange_is_empty(const cusd_PrimSiblingRange *iterator) {
        return reinterpret_cast<const pxr::UsdPrimSiblingRange *>(iterator)->empty();
    }

    cusd_Prim cusd_PrimSiblingRange_next(cusd_PrimSiblingRange *iterator) {
        cusd_Prim prim = transmute<cusd_Prim>(reinterpret_cast<pxr::UsdPrimSiblingRange *>(iterator)->front());
        reinterpret_cast<pxr::UsdPrimSiblingRange *>(iterator)->advance_begin(1);
        return prim;
    }

    const char *cusd_Prim_get_type_name(const cusd_Prim *prim) {
        return reinterpret_cast<const pxr::UsdPrim *>(prim)->GetTypeName().data();
    }

    const char *cusd_Attribute_get_type_name(const cusd_Attribute *attribute) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->GetTypeName().GetCPPTypeName().data();
    }

    bool cusd_Prim_get_attribute(const cusd_Prim *prim, const cusd_Token *token, cusd_Attribute *out_attribute) {
        pxr::UsdAttribute attribute =
            reinterpret_cast<const pxr::UsdPrim *>(prim)->GetAttribute(*reinterpret_cast<const pxr::TfToken *>(token));
        if (!attribute) {
            return false;
        }

        *reinterpret_cast<pxr::UsdAttribute *>(out_attribute) = attribute;

        return true;
    }

    bool cusd_Attribute_has_authored_connections(const cusd_Attribute *attribute) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->HasAuthoredConnections();
    }

    bool cusd_Attribute_has_authored_value(const cusd_Attribute *attribute) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->HasAuthoredValue();
    }

    bool cusd_Attribute_has_value(const cusd_Attribute *attribute) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->HasValue();
    }

    bool cusd_Attribute_is_authored(const cusd_Attribute *attribute) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->IsAuthored();
    }

    cusd_AttributeVector cusd_Prim_get_attributes(const cusd_Prim *prim) {
        return transmute<cusd_AttributeVector>(reinterpret_cast<const pxr::UsdPrim *>(prim)->GetAttributes());
    }

    bool cusd_Attribute_get_double(const cusd_Attribute *attribute, double *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(value);
    }

    cusd_GeomXformCache cusd_GeomXformCache_new() {
        return {};
    }

    void cusd_GeomXformCache_get_transform(cusd_GeomXformCache *cache, const cusd_Prim *prim, double *array) {
        pxr::GfMatrix4d transform = reinterpret_cast<pxr::UsdGeomXformCache *>(cache)->GetLocalToWorldTransform(
            *reinterpret_cast<const pxr::UsdPrim *>(prim));
        memcpy(array, transform.data(), sizeof(double) * 16);
    }

    cusd_VtArray cusd_Attribute_get_int_array(const cusd_Attribute *attribute) {
        pxr::VtArray<int> value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return transmute<cusd_VtArray>(std::move(value));
    }

    cusd_VtArray cusd_Attribute_get_int64_array(const cusd_Attribute *attribute) {
        pxr::VtArray<int64_t> value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return transmute<cusd_VtArray>(std::move(value));
    }

    cusd_Token cusd_Attribute_get_token(const cusd_Attribute *attribute) {
        pxr::TfToken value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return transmute<cusd_Token>(value);
    }

    bool cusd_Attribute_get_token_metadata(const cusd_Attribute *attribute, const cusd_Token *name,
                                           cusd_Token *out_token) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->GetMetadata(
            *reinterpret_cast<const pxr::TfToken *>(name), reinterpret_cast<pxr::TfToken *>(out_token));
    }

    bool cusd_Attribute_get_vec3f_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<pxr::GfVec3f> *>(value));
    }

    bool cusd_Attribute_get_vec2f_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<pxr::GfVec2f> *>(value));
    }

    bool cusd_Attribute_get_vec3d_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<pxr::GfVec3d> *>(value));
    }

    bool cusd_Attribute_get_vec2d_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<pxr::GfVec2d> *>(value));
    }

    bool cusd_Attribute_get_float_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<float> *>(value));
    }

    bool cusd_Attribute_get_token_array(const cusd_Attribute *attribute, cusd_VtArray *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(
            reinterpret_cast<pxr::VtArray<pxr::TfToken> *>(value));
    }

    cusd_VtArray cusd_Attribute_get_double_array(const cusd_Attribute *attribute) {
        pxr::VtArray<double> value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return transmute<cusd_VtArray>(std::move(value));
    }

    cusd_VtArray cusd_Attribute_get_string_array(const cusd_Attribute *attribute) {
        pxr::VtArray<std::string> value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return transmute<cusd_VtArray>(std::move(value));
    }

    bool cusd_Attribute_get_bool(const cusd_Attribute *attribute) {
        bool value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return value;
    }

    float cusd_Attribute_get_float(const cusd_Attribute *attribute) {
        float value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return value;
    }

    int cusd_Attribute_get_int(const cusd_Attribute *attribute) {
        int value;
        reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&value);
        return value;
    }

    bool cusd_Attribute_get_string(const cusd_Attribute *attribute, cusd_String *value) {
        std::string output_tmp;
        if (!reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&output_tmp)) {
            return false;
        }
        if (output_tmp.size() > 0) {
            *reinterpret_cast<std::string *>(value) = output_tmp;
        }

        return true;
    }

    bool cusd_Attribute_get_quatf(const cusd_Attribute *attribute, float *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfQuatf *>(value));
    }

    bool cusd_Attribute_get_vec4f(const cusd_Attribute *attribute, float *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfVec4f *>(value));
    }

    bool cusd_Attribute_get_vec3f(const cusd_Attribute *attribute, float *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfVec3f *>(value));
    }

    bool cusd_Attribute_get_vec2f(const cusd_Attribute *attribute, float *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfVec2f *>(value));
    }

    bool cusd_Attribute_get_vec3d(const cusd_Attribute *attribute, double *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfVec3d *>(value));
    }

    bool cusd_Attribute_get_vec2d(const cusd_Attribute *attribute, double *value) {
        return reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(reinterpret_cast<pxr::GfVec2d *>(value));
    }

    cusd_Prim cusd_Prim_compute_bound_material(const cusd_Prim *prim) {
        pxr::UsdShadeMaterialBindingAPI api =
            pxr::UsdShadeMaterialBindingAPI(*reinterpret_cast<const pxr::UsdPrim *>(prim));
        pxr::UsdShadeMaterial material = api.ComputeBoundMaterial(pxr::UsdShadeTokens->full);
        return transmute<cusd_Prim>(material.GetPrim());
    }

    bool cusd_Prim_compute_material_surface_source(const cusd_Prim *material, cusd_Prim *shader_prim) {
        pxr::UsdShadeShader source =
            pxr::UsdShadeMaterial(*reinterpret_cast<const pxr::UsdPrim *>(material)).ComputeSurfaceSource();
        if (!source) {
            return false;
        }
        *reinterpret_cast<pxr::UsdPrim *>(shader_prim) = source.GetPrim();
        return true;
    }

    bool cusd_Attribute_get_resolved_path(const cusd_Attribute *attribute, cusd_String *out_path) {
        pxr::SdfAssetPath path;
        if (!reinterpret_cast<const pxr::UsdAttribute *>(attribute)->Get(&path)) {
            return false;
        }

        const std::string &resolved_path = path.GetResolvedPath();

        if (resolved_path.size() == 0) {
            return false;
        }

        *reinterpret_cast<std::string *>(out_path) = resolved_path;

        return true;
    }

    cusd_ShadeSourceInfoVector cusd_Attribute_get_connected_shade_sources(const cusd_Attribute *attribute) {
        return transmute<cusd_ShadeSourceInfoVector>(
            pxr::UsdShadeConnectableAPI::GetConnectedSources(*reinterpret_cast<const pxr::UsdAttribute *>(attribute)));
    }

    cusd_PrimSiblingRange cusd_Prim_get_all_children(const cusd_Prim *prim) {
        return transmute<cusd_PrimSiblingRange>(reinterpret_cast<const pxr::UsdPrim *>(prim)->GetAllChildren());
    }

    cusd_Prim cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo *source_info) {
        return transmute<cusd_Prim>(
            reinterpret_cast<const pxr::UsdShadeConnectionSourceInfo *>(source_info)->source.GetPrim());
    }

    cusd_Token cusd_Token_new(const char *text) {
        // printf("%p\n", text);
        return transmute<cusd_Token>(pxr::TfToken(text, pxr::TfToken::Immortal));
    }

    bool cusd_Token_equal(const cusd_Token *a, const cusd_Token *b) {
        return *reinterpret_cast<const pxr::TfToken *>(a) == *reinterpret_cast<const pxr::TfToken *>(b);
    }

    cusd_Token cusd_Attribute_get_base_name(const cusd_Attribute *attribute) {
        return transmute<cusd_Token>(reinterpret_cast<const pxr::UsdAttribute *>(attribute)->GetBaseName());
    }

    cusd_Token cusd_Attribute_get_namespace(const cusd_Attribute *attribute) {
        return transmute<cusd_Token>(reinterpret_cast<const pxr::UsdAttribute *>(attribute)->GetNamespace());
    }
}
