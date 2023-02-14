#include <pxr/usd/usd/stage.h>
#include <pxr/usd/usd/primRange.h>
#include <pxr/usd/usd/prim.h>
#include <pxr/usd/usd/attribute.h>
#include <pxr/usd/usdGeom/xformCache.h>
#include <pxr/usd/usdGeom/subset.h>
#include <pxr/usd/usdShade/materialBindingAPI.h>
#include <iostream>

extern "C" {

struct cusd_Stage {
    pxr::UsdStageRefPtr inner;
};

struct cusd_PrimRange {
    pxr::UsdPrimRange inner;
};

struct cusd_Prim {
    pxr::UsdPrim inner;
};

struct cusd_Attribute {
    pxr::UsdAttribute inner;
};

struct cusd_GeomXformCache {
    pxr::UsdGeomXformCache inner;
};

struct cusd_IntArray {
    pxr::VtArray<int> inner;
};

struct cusd_Vec3fArray {
    pxr::VtArray<pxr::GfVec3f> inner;
};

struct cusd_Vec2fArray {
    pxr::VtArray<pxr::GfVec2f> inner;
};

struct cusd_ShadeMaterial {
    pxr::UsdShadeMaterial inner;
};

struct cusd_GeomSubsetVector {
    std::vector<pxr::UsdGeomSubset> inner;
};

struct cusd_GeomSubset {
    pxr::UsdGeomSubset inner;
};

struct cusd_ShadeShader {
    pxr::UsdShadeShader inner;
};

struct cusd_ShadeInput {
    pxr::UsdShadeInput inner;
};

struct cusd_ShadeSourceInfoVector {
    pxr::UsdShadeSourceInfoVector inner;
};

struct cusd_ShadeConnectionSourceInfo {
    pxr::UsdShadeConnectionSourceInfo inner;
};

struct cusd_String {
    std::string inner;
};

struct cusd_Token {
    pxr::TfToken inner;
};

struct cusd_LayerVector {
    pxr::SdfLayerHandleVector inner;
};

struct cusd_Layer {
    pxr::SdfLayerHandle inner;
};

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

void cusd_Stage_free(const cusd_Stage& stage) {
    stage.~cusd_Stage();
}

cusd_LayerVector cusd_Stage_get_used_layers(const cusd_Stage& stage) {
    return {
        .inner = stage.inner->GetUsedLayers()
    };
}

cusd_Layer* cusd_LayerVector_pointer(const cusd_LayerVector& layers) {
    return (cusd_Layer*) layers.inner.data();
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

void cusd_Layer_free(const cusd_Layer& layer) {
    layer.~cusd_Layer();
}

size_t cusd_LayerVector_size(const cusd_LayerVector& layers) {
    return layers.inner.size();
}

bool cusd_PrimRange_is_empty(const cusd_PrimRange& iterator) {
    return iterator.inner.empty();
}

void cusd_PrimRange_free(const cusd_PrimRange& iterator) {
    iterator.~cusd_PrimRange();
}

void cusd_LayerVector_free(const cusd_LayerVector& vector) {
    return vector.~cusd_LayerVector();
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

void cusd_Prim_free(const cusd_Prim& prim) {
    prim.~cusd_Prim();
}

const char* cusd_Attribute_get_type_name(const cusd_Attribute& attribute) {
    return attribute.inner.GetTypeName().GetCPPTypeName().data();
}

void cusd_Attribute_free(const cusd_Attribute& attribute) {
    return attribute.~cusd_Attribute();
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

void cusd_GeomXformCache_free(const cusd_GeomXformCache& cache) {
    cache.~cusd_GeomXformCache();
}

cusd_IntArray cusd_Attribute_get_int_array(const cusd_Attribute& attribute) {
    pxr::VtArray<int> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

bool cusd_Attribute_get_token_metadata(const cusd_Attribute& attribute, const cusd_Token& name, cusd_Token& out_token) {
    return attribute.inner.GetMetadata(name.inner, &out_token.inner);
}

size_t cusd_IntArray_size(const cusd_IntArray& array) {
    return array.inner.size();
}

const int* cusd_IntArray_pointer(const cusd_IntArray& array) {
    return array.inner.data();
}

void cusd_IntArray_free(const cusd_IntArray& array) {
    array.~cusd_IntArray();
}

cusd_Vec3fArray cusd_Attribute_get_vec3f_array(const cusd_Attribute& attribute) {
    pxr::VtArray<pxr::GfVec3f> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

size_t cusd_Vec3fArray_size(const cusd_Vec3fArray& array) {
    return array.inner.size() * 3;
}

const float* cusd_Vec3fArray_pointer(const cusd_Vec3fArray& array) {
    return (float*) array.inner.data();
}

void cusd_Vec3fArray_free(const cusd_Vec3fArray& array) {
    array.~cusd_Vec3fArray();
}

cusd_Vec2fArray cusd_Attribute_get_vec2f_array(const cusd_Attribute& attribute) {
    pxr::VtArray<pxr::GfVec2f> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

size_t cusd_Vec2fArray_size(const cusd_Vec2fArray& array) {
    return array.inner.size() * 2;
}

const float* cusd_Vec2fArray_pointer(const cusd_Vec2fArray& array) {
    return (float*) array.inner.data();
}

void cusd_Vec2fArray_free(const cusd_Vec2fArray& array) {
    array.~cusd_Vec2fArray();
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

void cusd_ShadeMaterial_free(const cusd_ShadeMaterial& material) {
    material.~cusd_ShadeMaterial();
}

bool cusd_ShadeShader_get_input(const cusd_ShadeShader& shader, const cusd_Token& name, cusd_ShadeInput& out_input) {
    auto input = shader.inner.GetInput(name.inner);

    if (!input) {
        return false;
    }
    
    out_input.inner = input;
    
    return true;
}

void cusd_ShadeShader_free(const cusd_ShadeShader& shader) {
    shader.~cusd_ShadeShader();
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

void cusd_ShadeInput_free(const cusd_ShadeInput& input) {
    input.~cusd_ShadeInput();
}

cusd_GeomSubsetVector cusd_Prim_get_all_subsets(const cusd_Prim& prim) {
    return {
        .inner = pxr::UsdGeomSubset::GetAllGeomSubsets(pxr::UsdGeomImageable(prim.inner))
    };
}

size_t cusd_GeomSubsetVector_size(const cusd_GeomSubsetVector& subsets) {
    return subsets.inner.size();
}

const cusd_GeomSubset* cusd_GeomSubsetVector_pointer(const cusd_GeomSubsetVector& subsets) {
    return (cusd_GeomSubset*) subsets.inner.data();
}

void cusd_GeomSubsetVector_free(const cusd_GeomSubsetVector& vector) {
    vector.~cusd_GeomSubsetVector();
}

cusd_Attribute cusd_GeomSubset_get_indices_attr(const cusd_GeomSubset& subset) {
    return {
        .inner = subset.inner.GetIndicesAttr()
    };
}

void cusd_GeomSubset_free(const cusd_GeomSubset& subset) {
    subset.~cusd_GeomSubset();
}

size_t cusd_ShadeSourceInfoVector_size(const cusd_ShadeSourceInfoVector& source_info) {
    return source_info.inner.size();
}

cusd_ShadeConnectionSourceInfo* cusd_ShadeSourceInfoVector_pointer(const cusd_ShadeSourceInfoVector& source_info) {
    return (cusd_ShadeConnectionSourceInfo*) source_info.inner.data();
}

void cusd_ShadeSourceInfoVector_free(const cusd_ShadeSourceInfoVector& vector) {
    return vector.~cusd_ShadeSourceInfoVector();
}

cusd_ShadeShader cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo& source_info) {
    return {.inner=source_info.inner.source};
}

void cusd_ShadeConnectionSourceInfo_free(const cusd_ShadeConnectionSourceInfo& source_info) {
    source_info.~cusd_ShadeConnectionSourceInfo();
}

const char* cusd_String_pointer(const cusd_String& string) {
    return string.inner.data();
}

size_t cusd_String_size(const cusd_String& string) {
    return string.inner.size();
}

void cusd_String_free(const cusd_String& string) {
    string.~cusd_String();
}

const char* cusd_Token_pointer(const cusd_Token& token) {
    return token.inner.data();
}

size_t cusd_Token_size(const cusd_Token& token) {
    return token.inner.size();
}

void cusd_Token_free(const cusd_Token& token) {
    token.~cusd_Token();
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
