#include <pxr/usd/usd/stage.h>
#include <pxr/usd/usd/primRange.h>
#include <pxr/usd/usd/prim.h>
#include <pxr/usd/usd/attribute.h>
#include <pxr/usd/usdGeom/xformCache.h>
#include <pxr/usd/usdGeom/subset.h>
#include <pxr/usd/usdShade/materialBindingAPI.h>
#include <iostream>

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
        return (const ITEM_TYPE *) binding.inner.data();                \
    }                                                                   \
                                                                        \
    size_t NAME##_size(const NAME& binding) {                           \
        return binding.inner.size() * SIZE_MULTIPLIER;                  \
    }                                       


extern "C" {

GENERATE_BINDING(cusd_Stage, pxr::UsdStageRefPtr);

GENERATE_BINDING(cusd_PrimRange, pxr::UsdPrimRange);

GENERATE_BINDING(cusd_Prim, pxr::UsdPrim);

GENERATE_BINDING(cusd_Attribute, pxr::UsdAttribute);

GENERATE_BINDING(cusd_GeomXformCache, pxr::UsdGeomXformCache);

GENERATE_BINDING(cusd_IntArray, pxr::VtArray<int>);

GENERATE_POINTER_AND_SIZE_FNS(cusd_IntArray, int, 1);

GENERATE_BINDING(cusd_Vec3fArray, pxr::VtArray<pxr::GfVec3f>);

GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec3fArray, float, 3);

GENERATE_BINDING(cusd_Vec2fArray, pxr::VtArray<pxr::GfVec2f>);

GENERATE_POINTER_AND_SIZE_FNS(cusd_Vec2fArray, float, 2);

GENERATE_BINDING(cusd_ShadeMaterial, pxr::UsdShadeMaterial);

GENERATE_BINDING(cusd_GeomSubset, pxr::UsdGeomSubset);

GENERATE_BINDING(cusd_GeomSubsetVector, std::vector<pxr::UsdGeomSubset>);

GENERATE_POINTER_AND_SIZE_FNS(cusd_GeomSubsetVector, cusd_GeomSubset, 1);

GENERATE_BINDING(cusd_ShadeShader, pxr::UsdShadeShader);

GENERATE_BINDING(cusd_ShadeInput, pxr::UsdShadeInput);

GENERATE_BINDING(cusd_ShadeConnectionSourceInfo, pxr::UsdShadeConnectionSourceInfo);

GENERATE_BINDING(cusd_ShadeSourceInfoVector, pxr::UsdShadeSourceInfoVector);

GENERATE_POINTER_AND_SIZE_FNS(cusd_ShadeSourceInfoVector, cusd_ShadeConnectionSourceInfo, 1);

GENERATE_BINDING(cusd_String, std::string);

GENERATE_POINTER_AND_SIZE_FNS(cusd_String, char, 1);

GENERATE_BINDING(cusd_Token, pxr::TfToken);

GENERATE_POINTER_AND_SIZE_FNS(cusd_Token, char, 1);

GENERATE_BINDING(cusd_Layer, pxr::SdfLayerHandle);

GENERATE_BINDING(cusd_LayerVector, pxr::SdfLayerHandleVector);

GENERATE_POINTER_AND_SIZE_FNS(cusd_LayerVector, cusd_Layer, 1);

cusd_Stage* cusd_Stage_open(const char* filename) {
    // Allocate the stage on the heap.
    return new cusd_Stage {
        .inner = std::move(pxr::UsdStage::Open(filename))
    };
}

const cusd_PrimRange cusd_Stage_iter_prims(const cusd_Stage& stage) {
    return {
        .inner = std::move(pxr::UsdPrimRange::Stage(stage.inner))
    };
}

void cusd_Stage_reload(const cusd_Stage& stage) {
    return stage.inner->Reload();
}

cusd_LayerVector cusd_Stage_get_used_layers(const cusd_Stage& stage) {
    return {
        .inner = stage.inner->GetUsedLayers()
    };
}

bool cusd_Layer_get_path(const cusd_Layer& layer, cusd_String& out_string) {
    auto& path = layer.inner->GetResolvedPath();

    if (!path) {
        return false;
    }

    out_string.inner = path.GetPathString();

    return true;
}

bool cusd_Layer_reload(const cusd_Layer& layer) {
    return layer.inner->Reload();
}

bool cusd_PrimRange_is_empty(const cusd_PrimRange& iterator) {
    return iterator.inner.empty();
}

cusd_Prim cusd_PrimRange_next(cusd_PrimRange& iterator) {
    cusd_Prim prim = {
        .inner = iterator.inner.front()
    };
    iterator.inner.increment_begin();
    return prim;
}

const char* cusd_Prim_get_type_name(const cusd_Prim& prim) {
    return prim.inner.GetTypeName().data();
}

const char* cusd_Attribute_get_type_name(const cusd_Attribute& attribute) {
    return attribute.inner.GetTypeName().GetCPPTypeName().data();
}

bool cusd_Prim_get_attribute(const cusd_Prim& prim, const cusd_Token& token, cusd_Attribute& out_attribute) {
    auto attribute = prim.inner.GetAttribute(token.inner);
    if (!attribute) {
        return false;
    }

    out_attribute.inner = attribute;

    return true;
}

double cusd_Attribute_get_double(const cusd_Attribute& attribute) {
    double value;
    attribute.inner.Get(&value);
    return value;
}

cusd_GeomXformCache cusd_GeomXformCache_new() {
    return {};
}

void cusd_GeomXformCache_get_transform(cusd_GeomXformCache& cache, const cusd_Prim& prim, double* array) {
    auto transform = cache.inner.GetLocalToWorldTransform(prim.inner);
    memcpy(array, transform.data(), sizeof(double) * 16);
}

cusd_IntArray cusd_Attribute_get_int_array(const cusd_Attribute& attribute) {
    pxr::VtArray<int> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

bool cusd_Attribute_get_token_metadata(const cusd_Attribute& attribute, const cusd_Token& name, cusd_Token& out_token) {
    return attribute.inner.GetMetadata(name.inner, &out_token.inner);
}

cusd_Vec3fArray cusd_Attribute_get_vec3f_array(const cusd_Attribute& attribute) {
    pxr::VtArray<pxr::GfVec3f> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

cusd_Vec2fArray cusd_Attribute_get_vec2f_array(const cusd_Attribute& attribute) {
    pxr::VtArray<pxr::GfVec2f> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

cusd_ShadeMaterial cusd_Prim_compute_bound_material(const cusd_Prim& prim) {
    auto api = pxr::UsdShadeMaterialBindingAPI(prim.inner);
    auto material = api.ComputeBoundMaterial(pxr::UsdShadeTokens->full);
    return {.inner = material};
}

cusd_ShadeMaterial cusd_GeomSubset_compute_bound_material(const cusd_GeomSubset& subset) {
    auto api = pxr::UsdShadeMaterialBindingAPI(subset.inner);
    auto material = api.ComputeBoundMaterial(pxr::UsdShadeTokens->full);
    return {.inner = material};
}

bool cusd_ShadeMaterial_compute_surface_source(const cusd_ShadeMaterial& material, cusd_ShadeShader& shader) {
    auto source = material.inner.ComputeSurfaceSource();
    if (!source) {
        return false;
    }
    shader.inner = source;
    return true;
}

bool cusd_ShadeShader_get_input(const cusd_ShadeShader& shader, const cusd_Token& name, cusd_ShadeInput& out_input) {
    auto input = shader.inner.GetInput(name.inner);

    if (!input) {
        return false;
    }
    
    out_input.inner = input;
    
    return true;
}

bool cusd_ShadeInput_get_resolved_path(const cusd_ShadeInput& input, cusd_String& out_path) {
    pxr::SdfAssetPath path;
    if (!input.inner.Get(&path)) {
        return false;
    }

    auto& resolved_path = path.GetResolvedPath();

    if (resolved_path.size() == 0) {
        return false;
    }

    out_path.inner = resolved_path;

    return true;
}

cusd_ShadeSourceInfoVector cusd_ShadeInput_get_connected_sources(const cusd_ShadeInput& input) {
    return {
        .inner = input.inner.GetConnectedSources()
    };
}

cusd_GeomSubsetVector cusd_Prim_get_all_subsets(const cusd_Prim& prim) {
    return {
        .inner = pxr::UsdGeomSubset::GetAllGeomSubsets(pxr::UsdGeomImageable(prim.inner))
    };
}

cusd_Attribute cusd_GeomSubset_get_indices_attr(const cusd_GeomSubset& subset) {
    return {
        .inner = subset.inner.GetIndicesAttr()
    };
}

cusd_ShadeShader cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo& source_info) {
    return {.inner=source_info.inner.source};
}

cusd_Token* cusd_Token_new(const char* text) {
    // todo: had problems without this `new`.
    return new cusd_Token {
        .inner = pxr::TfToken(text)
    };
}

bool cusd_Token_equal(const cusd_Token& a, const cusd_Token& b) {
    return a.inner == b.inner;
}

}
