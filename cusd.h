#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

#define DEFINE_OPAQUE(NAME, OPAQUE_SIZE)                                                                               \
    typedef struct NAME {                                                                                              \
        uint64_t _opaque[OPAQUE_SIZE];                                                                                 \
    } NAME;                                                                                                            \
                                                                                                                       \
    void NAME##_free(const NAME *binding);

#define DEFINE_SLICE_FNS(NAME, ITEM_TYPE)                                                                              \
    size_t NAME##_size(const NAME *binding);                                                                           \
    const ITEM_TYPE *NAME##_pointer(const NAME *binding);

    DEFINE_OPAQUE(cusd_VtArray, 5);
    DEFINE_OPAQUE(cusd_Token, 1);
    DEFINE_OPAQUE(cusd_Stage, 1);
    DEFINE_OPAQUE(cusd_PrimRange, 7);
    DEFINE_OPAQUE(cusd_Prim, 4);
    DEFINE_OPAQUE(cusd_Attribute, 4);
    DEFINE_OPAQUE(cusd_GeomXformCache, 6);
    DEFINE_OPAQUE(cusd_ShadeConnectionSourceInfo, 7);
    // Note: internally this is a `TfSmallVector<UsdShadeConnectionSourceInfo, 1>`.
    DEFINE_OPAQUE(cusd_ShadeSourceInfoVector, 8);
    DEFINE_OPAQUE(cusd_String, 4);
    DEFINE_OPAQUE(cusd_Layer, 2);
    DEFINE_OPAQUE(cusd_LayerVector, 3);
    DEFINE_OPAQUE(cusd_PrimSiblingRange, 10);
    DEFINE_OPAQUE(cusd_AttributeVector, 3);

    DEFINE_SLICE_FNS(cusd_ShadeSourceInfoVector, cusd_ShadeConnectionSourceInfo);
    DEFINE_SLICE_FNS(cusd_VtArray, uint8_t);
    DEFINE_SLICE_FNS(cusd_String, char);
    DEFINE_SLICE_FNS(cusd_LayerVector, cusd_Layer);
    DEFINE_SLICE_FNS(cusd_AttributeVector, cusd_Attribute);
    DEFINE_SLICE_FNS(cusd_Token, char);

    cusd_Stage cusd_Stage_open(const char *filename);

    bool cusd_Attribute_has_authored_connections(const cusd_Attribute *attribute);
    bool cusd_Attribute_has_authored_value(const cusd_Attribute *attribute);
    bool cusd_Attribute_has_value(const cusd_Attribute *attribute);
    bool cusd_Attribute_is_authored(const cusd_Attribute *attribute);

    bool cusd_Attribute_get_token_metadata(const cusd_Attribute *attribute, const cusd_Token *name,
                                           cusd_Token *out_token);

    cusd_VtArray cusd_Attribute_get_int_array(const cusd_Attribute *attribute);
    cusd_VtArray cusd_Attribute_get_int64_array(const cusd_Attribute *attribute);
    cusd_Token cusd_Attribute_get_token(const cusd_Attribute *attribute);
    bool cusd_Attribute_get_vec3f_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    bool cusd_Attribute_get_vec2f_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    bool cusd_Attribute_get_vec3d_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    bool cusd_Attribute_get_vec2d_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    bool cusd_Attribute_get_float_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    bool cusd_Attribute_get_token_array(const cusd_Attribute *attribute, cusd_VtArray *value);
    cusd_VtArray cusd_Attribute_get_double_array(const cusd_Attribute *attribute);
    cusd_VtArray cusd_Attribute_get_string_array(const cusd_Attribute *attribute);
    bool cusd_Attribute_get_bool(const cusd_Attribute *attribute);
    float cusd_Attribute_get_float(const cusd_Attribute *attribute);
    bool cusd_Attribute_get_double(const cusd_Attribute *attribute, double *value);
    int cusd_Attribute_get_int(const cusd_Attribute *attribute);
    bool cusd_Attribute_get_string(const cusd_Attribute *attribute, cusd_String *value);
    bool cusd_Attribute_get_quatf(const cusd_Attribute *attribute, float *value);
    bool cusd_Attribute_get_vec4f(const cusd_Attribute *attribute, float *value);
    bool cusd_Attribute_get_vec3f(const cusd_Attribute *attribute, float *value);
    bool cusd_Attribute_get_vec2f(const cusd_Attribute *attribute, float *value);
    bool cusd_Attribute_get_vec3d(const cusd_Attribute *attribute, double *value);
    bool cusd_Attribute_get_vec2d(const cusd_Attribute *attribute, double *value);

    cusd_Token cusd_Token_new(const char *text);
    bool cusd_Token_equal(const cusd_Token *a, const cusd_Token *b);

    bool cusd_Attribute_get_resolved_path(const cusd_Attribute *attribute, cusd_String *out_path);
    cusd_Token cusd_Attribute_get_base_name(const cusd_Attribute *attribute);
    cusd_Token cusd_Attribute_get_namespace(const cusd_Attribute *attribute);

    cusd_GeomXformCache cusd_GeomXformCache_new();

    void cusd_GeomXformCache_get_transform(cusd_GeomXformCache *cache, const cusd_Prim *prim, double *array);

    const char *cusd_Prim_get_type_name(const cusd_Prim *prim);

    const char *cusd_Attribute_get_type_name(const cusd_Attribute *attribute);

    bool cusd_Prim_get_attribute(const cusd_Prim *prim, const cusd_Token *token, cusd_Attribute *out_attribute);

    cusd_AttributeVector cusd_Prim_get_attributes(const cusd_Prim *prim);

    const cusd_PrimRange cusd_Stage_iter_prims(const cusd_Stage *stage);

    void cusd_Stage_reload(const cusd_Stage *stage);

    cusd_LayerVector cusd_Stage_get_used_layers(const cusd_Stage *stage);

    bool cusd_Layer_get_path(const cusd_Layer *layer, cusd_String *out_string);

    bool cusd_Layer_reload(const cusd_Layer *layer);

    bool cusd_PrimRange_is_empty(const cusd_PrimRange *iterator);
    cusd_Prim cusd_PrimRange_next(cusd_PrimRange *iterator);

    bool cusd_PrimSiblingRange_is_empty(const cusd_PrimSiblingRange *iterator);
    cusd_Prim cusd_PrimSiblingRange_next(cusd_PrimSiblingRange *iterator);

    const char *cusd_Prim_get_type_name(const cusd_Prim *prim);

    const char *cusd_Attribute_get_type_name(const cusd_Attribute *attribute);

    cusd_ShadeSourceInfoVector cusd_Attribute_get_connected_shade_sources(const cusd_Attribute *attribute);

    cusd_PrimSiblingRange cusd_Prim_get_all_children(const cusd_Prim *prim);

    cusd_Prim cusd_ShadeConnectionSourceInfo_source(const cusd_ShadeConnectionSourceInfo *source_info);

    cusd_Prim cusd_Prim_compute_bound_material(const cusd_Prim *prim);

    bool cusd_Prim_compute_material_surface_source(const cusd_Prim *material, cusd_Prim *shader_prim);

#ifdef __cplusplus
}
#endif
