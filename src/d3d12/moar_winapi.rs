#![allow(non_camel_case_types, non_snake_case)]

use winapi::{
    ctypes::c_void,
    shared::{
        basetsd::{UINT16, UINT32, UINT64},
        dxgiformat::DXGI_FORMAT,
        dxgitype::DXGI_SAMPLE_DESC,
        guiddef::REFIID,
        minwindef::UINT,
        ntdef::HRESULT,
    },
    um::d3d12::*,
    RIDL,
};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub(super) enum D3D12_BARRIER_LAYOUT {
    D3D12_BARRIER_LAYOUT_UNDEFINED,
    D3D12_BARRIER_LAYOUT_COMMON,
    D3D12_BARRIER_LAYOUT_PRESENT,
    D3D12_BARRIER_LAYOUT_GENERIC_READ,
    D3D12_BARRIER_LAYOUT_RENDER_TARGET,
    D3D12_BARRIER_LAYOUT_UNORDERED_ACCESS,
    D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_WRITE,
    D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_READ,
    D3D12_BARRIER_LAYOUT_SHADER_RESOURCE,
    D3D12_BARRIER_LAYOUT_COPY_SOURCE,
    D3D12_BARRIER_LAYOUT_COPY_DEST,
    D3D12_BARRIER_LAYOUT_RESOLVE_SOURCE,
    D3D12_BARRIER_LAYOUT_RESOLVE_DEST,
    D3D12_BARRIER_LAYOUT_SHADING_RATE_SOURCE,
    D3D12_BARRIER_LAYOUT_VIDEO_DECODE_READ,
    D3D12_BARRIER_LAYOUT_VIDEO_DECODE_WRITE,
    D3D12_BARRIER_LAYOUT_VIDEO_PROCESS_READ,
    D3D12_BARRIER_LAYOUT_VIDEO_PROCESS_WRITE,
    D3D12_BARRIER_LAYOUT_VIDEO_ENCODE_READ,
    D3D12_BARRIER_LAYOUT_VIDEO_ENCODE_WRITE,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COMMON,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_GENERIC_READ,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_UNORDERED_ACCESS,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_SHADER_RESOURCE,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_SOURCE,
    D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_DEST,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COMMON,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_GENERIC_READ,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_UNORDERED_ACCESS,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_SHADER_RESOURCE,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_SOURCE,
    D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_DEST,
    D3D12_BARRIER_LAYOUT_VIDEO_QUEUE_COMMON,
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct D3D12_MIP_REGION {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub(super) struct D3D12_RESOURCE_DESC1 {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: UINT64,
    pub Width: UINT64,
    pub Height: UINT,
    pub DepthOrArraySize: UINT16,
    pub MipLevels: UINT16,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
    pub SamplerFeedbackMipRegion: D3D12_MIP_REGION,
}

RIDL! {
    #[uuid(0x81dadc15, 0x2bad, 0x4392, 0x93, 0xc5, 0x10, 0x13, 0x45, 0xc4, 0xaa, 0x98)]
    interface ID3D12Device3(ID3D12Device3Vtbl): ID3D12Device2(ID3D12Device2Vtbl) {
        // HRESULT OpenExistingHeapFromAddress(void *address,
        //         REFIID riid, void **heap);
        // fn OpenExistingHeapFromAddress(
        //     address: *const
        //     riid: REFIID,
        //     heap: *mut *mut c_void,
        // ) -> HRESULT,
        // HRESULT OpenExistingHeapFromFileMapping(HANDLE file_mapping,
        //         REFIID riid, void **heap);
        //
        // HRESULT EnqueueMakeResident(D3D12_RESIDENCY_FLAGS flags,
        //         UINT num_objects, ID3D12Pageable *const *objects,
        //         ID3D12Fence *fence_to_signal, UINT64 fence_value_to_signal);
    }
}

RIDL! {
    #[uuid(0xe865df17, 0xa9ee, 0x46f9, 0xa4, 0x63, 0x30, 0x98, 0x31, 0x5a, 0xa2, 0xe5)]
    interface ID3D12Device4(ID3D12Device4Vtbl): ID3D12Device3(ID3D12Device3Vtbl) {
        // HRESULT CreateCommandList1(UINT node_mask, D3D12_COMMAND_LIST_TYPE type,
        //         D3D12_COMMAND_LIST_FLAGS flags, REFIID riid, void **command_list);
        //
        // HRESULT CreateProtectedResourceSession(const D3D12_PROTECTED_RESOURCE_SESSION_DESC *desc,
        //         REFIID riid, void **session);
        //
        // HRESULT CreateCommittedResource1(const D3D12_HEAP_PROPERTIES *heap_properties, D3D12_HEAP_FLAGS heap_flags,
        //         const D3D12_RESOURCE_DESC *desc, D3D12_RESOURCE_STATES initial_state,
        //         const D3D12_CLEAR_VALUE *optimized_clear_value,
        //         ID3D12ProtectedResourceSession *protected_session,
        //         REFIID riid, void **resource);
        //
        // HRESULT CreateHeap1(const D3D12_HEAP_DESC *desc,
        //         ID3D12ProtectedResourceSession *protected_session,
        //         REFIID riid, void **heap);
        //
        // HRESULT CreateReservedResource1(const D3D12_RESOURCE_DESC *desc, D3D12_RESOURCE_STATES initial_state,
        //         const D3D12_CLEAR_VALUE *optimized_clear_value,
        //         ID3D12ProtectedResourceSession *protected_session,
        //         REFIID riid, void **resource);
        //
        // D3D12_RESOURCE_ALLOCATION_INFO GetResourceAllocationInfo1(UINT visible_mask,
        //         UINT reource_desc_count, const D3D12_RESOURCE_DESC *resource_descs,
        //         D3D12_RESOURCE_ALLOCATION_INFO1 *resource_allocation_infos);
    }
}

RIDL! {
    #[uuid(0x8b4f173b, 0x2fea, 0x4b80, 0x8f, 0x58, 0x43, 0x07, 0x19, 0x1a, 0xb9, 0x5d)]
    interface ID3D12Device5(ID3D12Device5Vtbl): ID3D12Device4(ID3D12Device4Vtbl) {
        // HRESULT CreateLifetimeTracker(ID3D12LifetimeOwner *owner, REFIID riid, void **tracker);
        //
        // void RemoveDevice();
        //
        // HRESULT EnumerateMetaCommands(UINT *num_meta_commands, D3D12_META_COMMAND_DESC *descs);
        //
        // HRESULT EnumerateMetaCommandParameters(REFGUID command_id, D3D12_META_COMMAND_PARAMETER_STAGE Stage,
        //         UINT *total_structure_size, UINT *parameter_count, D3D12_META_COMMAND_PARAMETER_DESC *parameter_descs);
        //
        // HRESULT CreateMetaCommand(REFGUID command_id, UINT node_mask, const void *creation_parameters_data,
        //         SIZE_T creation_parameters_size, REFIID riid, void **meta_command);
        //
        // HRESULT CreateStateObject(const D3D12_STATE_OBJECT_DESC *desc, REFIID riid, void **state_object);
        //
        // void GetRaytracingAccelerationStructurePrebuildInfo(
        //         const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS *desc,
        //         D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO *info);
        //
        // D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS CheckDriverMatchingIdentifier(
        //         D3D12_SERIALIZED_DATA_TYPE serialized_data_type,
        //         const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER *identifier);
    }
}

RIDL! {
    #[uuid(0xc70b221b, 0x40e4, 0x4a17, 0x89, 0xaf, 0x02, 0x5a, 0x07, 0x27, 0xa6, 0xdc)]
    interface ID3D12Device6(ID3D12Device6Vtbl): ID3D12Device5(ID3D12Device5Vtbl) {
        // HRESULT SetBackgroundProcessingMode(D3D12_BACKGROUND_PROCESSING_MODE mode,
        //         D3D12_MEASUREMENTS_ACTION action, HANDLE event, BOOL further_measurements);
    }
}

RIDL! {
    #[uuid(0x5c014b53, 0x68a1, 0x4b9b, 0x8b, 0xd1, 0xdd, 0x60, 0x46, 0xb9, 0x35, 0x8b)]
    interface ID3D12Device7(ID3D12Device7Vtbl): ID3D12Device6(ID3D12Device6Vtbl) {
        // HRESULT AddToStateObject(const D3D12_STATE_OBJECT_DESC *addition,
        //         ID3D12StateObject *state_object, REFIID riid, void **new_state_object);
        //
        // HRESULT CreateProtectedResourceSession1(
        //         const D3D12_PROTECTED_RESOURCE_SESSION_DESC1 *desc,
        //         REFIID riid, void **session);
    }
}

RIDL! {
    #[uuid(0x9218e6bb, 0xf944, 0x4f7e, 0xa7, 0x5c, 0xb1, 0xb2, 0xc7, 0xb7, 0x01, 0xf3)]
    interface ID3D12Device8(ID3D12Device8Vtbl): ID3D12Device7(ID3D12Device7Vtbl) {
        // D3D12_RESOURCE_ALLOCATION_INFO GetResourceAllocationInfo2(UINT visible_mask,
        //         UINT resource_desc_count, const D3D12_RESOURCE_DESC1 *resource_descs,
        //         D3D12_RESOURCE_ALLOCATION_INFO1 *resource_allocation_infos);
        //
        // HRESULT CreateCommittedResource2(const D3D12_HEAP_PROPERTIES *heap_properties,
        //         D3D12_HEAP_FLAGS heap_flags, const D3D12_RESOURCE_DESC1 *resource_desc,
        //         D3D12_RESOURCE_STATES initial_state, const D3D12_CLEAR_VALUE *optimized_clear_value,
        //         ID3D12ProtectedResourceSession *protected_session, REFIID riid, void **resource);
        //
        // HRESULT CreatePlacedResource1(ID3D12Heap *heap, UINT64 heap_offset,
        //         const D3D12_RESOURCE_DESC1 *resource_desc, D3D12_RESOURCE_STATES initial_state,
        //         const D3D12_CLEAR_VALUE *optimized_clear_value, REFIID riid, void **resource);
        //
        // void CreateSamplerFeedbackUnorderedAccessView(ID3D12Resource *target_resource,
        //         ID3D12Resource *feedback_resource, D3D12_CPU_DESCRIPTOR_HANDLE descriptor);
        //
        // void GetCopyableFootprints1(const D3D12_RESOURCE_DESC1 *resource_desc,
        //         UINT first_sub_resource, UINT sub_resource_count, UINT64 base_offset,
        //         D3D12_PLACED_SUBRESOURCE_FOOTPRINT *layouts, UINT *row_count,
        //         UINT64 *row_size, UINT64 *total_bytes);
    }
}

RIDL! {
    #[uuid(0x4c80e962, 0xf032, 0x4f60, 0xbc, 0x9e, 0xeb, 0xc2, 0xcf, 0xa1, 0xd8, 0x3c)]
    interface ID3D12Device9(ID3D12Device9Vtbl): ID3D12Device8(ID3D12Device8Vtbl) {
        // HRESULT CreateShaderCacheSession(const D3D12_SHADER_CACHE_SESSION_DESC *desc,
        //         REFIID riid, void **session);
        //
        // HRESULT ShaderCacheControl(D3D12_SHADER_CACHE_KIND_FLAGS kinds,
        //         D3D12_SHADER_CACHE_CONTROL_FLAGS control);
        //
        // HRESULT CreateCommandQueue1(const D3D12_COMMAND_QUEUE_DESC *desc,
        //         REFIID creator_id, REFIID riid, void **command_queue);
    }
}

RIDL! {
    #[uuid(0x517f8718, 0xaa66, 0x49f9, 0xb0, 0x2b, 0xa7, 0xab, 0x89, 0xc0, 0x60, 0x31)]
    interface ID3D12Device10(ID3D12Device10Vtbl): ID3D12Device9(ID3D12Device9Vtbl) {
        fn CreateCommittedResource3(
            heap_properties: *const D3D12_HEAP_PROPERTIES,
            heap_flags: D3D12_HEAP_FLAGS,
            desc: *const D3D12_RESOURCE_DESC1,
            initial_layout: D3D12_BARRIER_LAYOUT,
            optimized_clear_value: *const D3D12_CLEAR_VALUE,
            protected_session: *mut ID3D12ProtectedResourceSession,
            num_castable_formats: UINT32,
            castable_formats: *const DXGI_FORMAT,
            iid: REFIID,
            resource: *mut *mut c_void,
        ) -> HRESULT,

        fn CreatePlacedResource2(
            heap: *mut ID3D12Heap,
            heap_offset: UINT64,
            desc: *const D3D12_RESOURCE_DESC1,
            initial_layout: D3D12_BARRIER_LAYOUT,
            optimized_clear_value: *const D3D12_CLEAR_VALUE,
            num_castable_formats: UINT32,
            castable_formats: *const DXGI_FORMAT,
            iid: REFIID,
            resource: *mut *mut c_void,
        ) -> HRESULT,

        // HRESULT CreateReservedResource2(const D3D12_RESOURCE_DESC *desc,
        //     D3D12_BARRIER_LAYOUT initial_layout,
        //     const D3D12_CLEAR_VALUE *optimized_clear_value,
        //     ID3D12ProtectedResourceSession *protected_session,
        //     UINT32 num_castable_formats,
        //     const DXGI_FORMAT *castable_formats,
        //     REFIID iid,
        //     void **resource);
    }
}
