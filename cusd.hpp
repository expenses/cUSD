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

    // As stages contain a reference counted pointer, we need to ensure that destructors
    // aren't called upon passing memory to rust.
    private:
        ~cusd_Stage() {};
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

cusd_Attribute cusd_Prim_get_attribute(const cusd_Prim& prim, const char* name) {
    return {
        .inner = prim.inner.GetAttribute(pxr::TfToken(name))
    };
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

size_t cusd_IntArray_size(const cusd_IntArray& array) {
    return array.inner.size();
}

const int* cusd_IntArray_pointer(const cusd_IntArray& array) {
    return array.inner.data();
}

cusd_Vec3fArray cusd_Attribute_get_vec3f_array(const cusd_Attribute& attribute) {
    pxr::VtArray<pxr::GfVec3f> value;
    attribute.inner.Get(&value);
    return { .inner = value };
}

size_t cusd_Vec3fArray_size(const cusd_Vec3fArray& array) {
    return array.inner.size();
}

const float* cusd_Vec3fArray_pointer(const cusd_Vec3fArray& array) {
    return (float*) array.inner.data();
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

cusd_ShadeInput cusd_ShadeShader_get_input(const cusd_ShadeShader& shader, const char* name) {
    return {
        .inner = shader.inner.GetInput(pxr::TfToken(name))
    };
}

cusd_String cusd_ShadeInput_get_resolved_path(const cusd_ShadeInput& input) {
    pxr::SdfAssetPath path;
    input.inner.Get(&path);
    return {.inner = path.GetResolvedPath()};
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

size_t cusd_GeomSubsetVector_size(const cusd_GeomSubsetVector& subsets) {
    return subsets.inner.size();
}

const cusd_GeomSubset* cusd_GeomSubsetVector_pointer(const cusd_GeomSubsetVector& subsets) {
    return (cusd_GeomSubset*) subsets.inner.data();
}

cusd_Attribute cusd_GeomSubset_get_indices_attr(const cusd_GeomSubset& subset) {
    return {
        .inner = subset.inner.GetIndicesAttr()
    };
}

size_t cusd_ShadeSourceInfoVector_size(const cusd_ShadeSourceInfoVector& source_info) {
    return source_info.inner.size();
}

cusd_ShadeConnectionSourceInfo* cusd_ShadeSourceInfoVector_pointer(const cusd_ShadeSourceInfoVector& source_info) {
    return (cusd_ShadeConnectionSourceInfo*) source_info.inner.data();
}

cusd_ShadeShader cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo& source_info) {
    return {.inner=source_info.inner.source};
}

const char* cusd_String_pointer(const cusd_String& string) {
    return string.inner.data();
}

size_t cusd_String_size(const cusd_String& string) {
    return string.inner.size();
}

}
