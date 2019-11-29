
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

// @see https://www.khronos.org/registry/vulkan/specs/1.1/html/vkspec.html

pub type VkDeviceSize = u64;

pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkSampleCountFlags = VkFlags;
pub type VkQueueFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkBufferCreateFlags = VkFlags;
pub type VkBufferUsageFlags = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkQueryControlFlags = VkFlags;
pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkFenceCreateFlags = VkFlags;
pub type VkPipelineStageFlags = VkFlags;
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkAccessFlags = VkFlags;
pub type VkDependencyFlags = VkFlags;

#[repr(C)]
pub struct VkInstanceOpaque { _private: [u8; 0] }
pub type VkInstance = *mut VkInstanceOpaque;
#[repr(C)]
pub struct VkPhysicalDeviceOpaque { _private: [u8; 0] }
pub type VkPhysicalDevice = *mut VkPhysicalDeviceOpaque;
#[repr(C)]
pub struct VkDeviceOpaque { _private: [u8; 0] }
pub type VkDevice = *mut VkDeviceOpaque;
#[repr(C)]
pub struct VkQueueOpaque { _private: [u8; 0] }
pub type VkQueue = *mut VkQueueOpaque;
#[repr(C)]
pub struct VkCommandPoolOpaque { _private: [u8; 0] }
pub type VkCommandPool = *mut VkCommandPoolOpaque;
#[repr(C)]
pub struct VkBufferOpaque { _private: [u8; 0] }
pub type VkBuffer = *mut VkBufferOpaque;
#[repr(C)]
pub struct VkDeviceMemoryOpaque { _private: [u8; 0] }
pub type VkDeviceMemory = *mut VkDeviceMemoryOpaque;
#[repr(C)]
pub struct VkCommandBufferOpaque { _private: [u8; 0] }
pub type VkCommandBuffer = *mut VkCommandBufferOpaque;
#[repr(C)]
pub struct VkRenderPassOpaque { _private: [u8; 0] }
pub type VkRenderPass = *mut VkRenderPassOpaque;
#[repr(C)]
pub struct VkFramebufferOpaque { _private: [u8; 0] }
pub type VkFramebuffer = *mut VkFramebufferOpaque;
#[repr(C)]
pub struct VkFenceOpaque { _private: [u8; 0] }
pub type VkFence = *mut VkFenceOpaque;
#[repr(C)]
pub struct VkSemaphoreOpaque { _private: [u8; 0] }
pub type VkSemaphore = *mut VkSemaphoreOpaque;
#[repr(C)]
pub struct VkDescriptorPoolOpaque { _private: [u8; 0] }
pub type VkDescriptorPool = *mut VkDescriptorPoolOpaque;
#[repr(C)]
pub struct VkDescriptorSetLayoutOpaque { _private: [u8; 0] }
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayoutOpaque;
#[repr(C)]
pub struct VkSamplerOpaque { _private: [u8; 0] }
pub type VkSampler = *mut VkSamplerOpaque;
#[repr(C)]
pub struct VkPipelineLayoutOpaque { _private: [u8; 0] }
pub type VkPipelineLayout = *mut VkPipelineLayoutOpaque;
#[repr(C)]
pub struct VkDescriptorSetOpaque { _private: [u8; 0] }
pub type VkDescriptorSet = *mut VkDescriptorSetOpaque;
#[repr(C)]
pub struct VkBufferViewOpaque { _private: [u8; 0] }
pub type VkBufferView = *mut VkBufferViewOpaque;
#[repr(C)]
pub struct VkImageViewOpaque { _private: [u8; 0] }
pub type VkImageView = *mut VkImageViewOpaque;
#[repr(C)]
pub struct VkPipelineCacheOpaque { _private: [u8; 0] }
pub type VkPipelineCache = *mut VkPipelineCacheOpaque;
#[repr(C)]
pub struct VkPipelineOpaque { _private: [u8; 0] }
pub type VkPipeline = *mut VkPipelineOpaque;
#[repr(C)]
pub struct VkShaderModuleOpaque { _private: [u8; 0] }
pub type VkShaderModule = *mut VkShaderModuleOpaque;
#[repr(C)]
pub struct VkMemoryBarrierOpaque { _private: [u8; 0] }
pub type VkMemoryBarrier = *mut VkMemoryBarrierOpaque;
#[repr(C)]
pub struct VkImageMemoryBarrierOpaque { _private: [u8; 0] }
pub type VkImageMemoryBarrier = *mut VkImageMemoryBarrierOpaque;

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: size_t = 256;
pub const VK_UUID_SIZE: size_t = 16;
pub const VK_MAX_MEMORY_TYPES: size_t = 32;
pub const VK_MAX_MEMORY_HEAPS: size_t = 16;
pub const VK_WHOLE_SIZE: u64 = u64::max_value();
pub const VK_FLAGS_NONE: VkFlags = 0;
pub const VK_TRUE: VkBool32 = 1;
pub const VK_FALSE: VkBool32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = u32::max_value();

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkResult.html
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VkResult {
    VK_SUCCESS = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_EVENT_SET = 3,
    VK_EVENT_RESET = 4,
    VK_INCOMPLETE = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    VK_ERROR_FRAGMENTED_POOL = -12,
    VK_ERROR_OUT_OF_POOL_MEMORY = -1000069000,
    VK_ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
    VK_SUBOPTIMAL_KHR = 1000001003,
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
    VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
    VK_ERROR_INVALID_SHADER_NV = -1000012000,
    VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT = -1000158000,
    VK_ERROR_FRAGMENTATION_EXT = -1000161000,
    VK_ERROR_NOT_PERMITTED_EXT = -1000174001,
    VK_ERROR_INVALID_DEVICE_ADDRESS_EXT = -1000244000,
    VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT = -1000255000,
    VK_RESULT_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExtent3D.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSharingMode.html
#[repr(C)]
pub enum VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
    VK_SHARING_MODE_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkStructureType.html
#[repr(C)]
pub enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
    VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkApplicationInfo.html
#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkInstanceCreateInfo.html
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAllocationCallbacks.html
pub enum VkAllocationCallbacks {}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceType.html
#[repr(C)]
pub enum VkPhysicalDeviceType {
    VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
    VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceLimits.html
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: c_float,
    pub maxSamplerAnisotropy: c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [c_float; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: c_float,
    pub maxInterpolationOffset: c_float,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [c_float; 2],
    pub lineWidthRange: [c_float; 2],
    pub pointSizeGranularity: c_float,
    pub lineWidthGranularity: c_float,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceSparseProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceQueueCreateInfo.html
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const c_float,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueueFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkQueueFlagBits {
    VK_QUEUE_GRAPHICS_BIT = 0x00000001,
    VK_QUEUE_COMPUTE_BIT = 0x00000002,
    VK_QUEUE_TRANSFER_BIT = 0x00000004,
    VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
    VK_QUEUE_PROTECTED_BIT = 0x00000010,
    VK_QUEUE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolCreateFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkCommandPoolCreateFlagBits {
    VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    VK_COMMAND_POOL_CREATE_PROTECTED_BIT = 0x00000004,
    VK_COMMAND_POOL_CREATE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueueFamilyProperties.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceFeatures.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceCreateInfo.html
#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolCreateInfo.html
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCreateInfo.html
#[repr(C)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryPropertyFlagBits.html
#[repr(C)]
pub enum VkMemoryPropertyFlagBits {
    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
    VK_MEMORY_PROPERTY_PROTECTED_BIT = 0x00000020,
    VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD = 0x00000040,
    VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD = 0x00000080,
    VK_MEMORY_PROPERTY_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryHeapFlagBits.html
#[repr(C)]
pub enum VkMemoryHeapFlagBits {
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_HEAP_MULTI_INSTANCE_BIT = 0x00000002,
    VK_MEMORY_HEAP_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryType.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryHeap.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMemoryProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryRequirements.html
#[repr(C)]
#[derive(Debug)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryAllocateInfo.html
#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferUsageFlagBits.html
#[repr(C)]
pub enum VkBufferUsageFlagBits {
    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x00000800,
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x00001000,
    VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00000200,
    VK_BUFFER_USAGE_RAY_TRACING_BIT_NV = 0x00000400,
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT = 0x00020000,
    VK_BUFFER_USAGE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMappedMemoryRange.html
#[repr(C)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferLevel.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
    VK_COMMAND_BUFFER_LEVEL_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferAllocateInfo.html
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferInheritanceInfo.html
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPipelineStatisticFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkQueryPipelineStatisticFlagBits {
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
    VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
    VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
    VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
    VK_QUERY_PIPELINE_STATISTIC_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferBeginInfo.html
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCopy.html
#[repr(C)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFenceCreateInfo.html
#[repr(C)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSubmitInfo.html
#[repr(C)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorType.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkDescriptorType {
    VK_DESCRIPTOR_TYPE_SAMPLER = 0,
    VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
    VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT = 1000138000,
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
    VK_DESCRIPTOR_TYPE_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorPoolSize.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize {
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorPoolCreateInfo.html
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetLayoutBinding.html
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineLayoutCreateInfo.html
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPushConstantRange.html
#[repr(C)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetAllocateInfo.html
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkWriteDescriptorSet.html
#[repr(C)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorImageInfo.html
#[repr(C)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorBufferInfo.html
#[repr(C)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCopyDescriptorSet.html
#[repr(C)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageLayout.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkImageLayout {
    VK_IMAGE_LAYOUT_UNDEFINED = 0,
    VK_IMAGE_LAYOUT_GENERAL = 1,
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,
    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
    VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR = 1000111000,
    VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV = 1000164003,
    VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT = 1000218000,
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR = 1000241000,
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR = 1000241001,
    VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR = 1000241002,
    VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR = 1000241003,
    VK_IMAGE_LAYOUT_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkShaderStageFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkShaderStageFlagBits {
    VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
    VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
    VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
    VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
    VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
    VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
    VK_SHADER_STAGE_RAYGEN_BIT_NV = 0x00000100,
    VK_SHADER_STAGE_ANY_HIT_BIT_NV = 0x00000200,
    VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV = 0x00000400,
    VK_SHADER_STAGE_MISS_BIT_NV = 0x00000800,
    VK_SHADER_STAGE_INTERSECTION_BIT_NV = 0x00001000,
    VK_SHADER_STAGE_CALLABLE_BIT_NV = 0x00002000,
    VK_SHADER_STAGE_TASK_BIT_NV = 0x00000040,
    VK_SHADER_STAGE_MESH_BIT_NV = 0x00000080,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkComputePipelineCreateInfo.html
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineShaderStageCreateInfo.html
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCacheCreateInfo.html
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: size_t,
    pub pInitialData: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationInfo.html
#[repr(C)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: size_t,
    pub pData: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationMapEntry.html
#[repr(C)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: size_t,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkShaderModuleCreateInfo.html
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: size_t,
    pub pCode: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFenceCreateFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkFenceCreateFlagBits {
    VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
    VK_FENCE_CREATE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAccessFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkAccessFlagBits {
    VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
    VK_ACCESS_INDEX_READ_BIT = 0x00000002,
    VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
    VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
    VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
    VK_ACCESS_SHADER_READ_BIT = 0x00000020,
    VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
    VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
    VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
    VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
    VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
    VK_ACCESS_HOST_READ_BIT = 0x00002000,
    VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
    VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
    VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,
    VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
    VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
    VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX = 0x00020000,
    VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX = 0x00040000,
    VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
    VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000,
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV = 0x00200000,
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV = 0x00400000,
    VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
    VK_ACCESS_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineStageFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkPipelineStageFlagBits {
    VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
    VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
    VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
    VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
    VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
    VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
    VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
    VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
    VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
    VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
    VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
    VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
    VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
    VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
    VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
    VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
    VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
    VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX = 0x00020000,
    VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV = 0x00400000,
    VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV = 0x00200000,
    VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV = 0x02000000,
    VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV = 0x00080000,
    VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV = 0x00100000,
    VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
    VK_PIPELINE_STAGE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferMemoryBarrier.html
#[repr(C)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineBindPoint.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,
    VK_PIPELINE_BIND_POINT_RAY_TRACING_NV = 1000165000,
    VK_PIPELINE_BIND_POINT_MAX_ENUM = 0x7FFFFFFF
}

#[link(name = "vulkan")]
extern "C" {
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateInstance.html
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEnumeratePhysicalDevices.html
    pub fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceProperties.html
    pub fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDevice.html
    pub fn vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetDeviceQueue.html
    pub fn vkGetDeviceQueue(
        device: VkDevice,
        queueFamilyIndex: u32,
        queueIndex: u32,
        queue: *mut VkQueue,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateCommandPool.html
    pub fn vkCreateCommandPool(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        commandPool: *mut VkCommandPool,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateBuffer.html
    pub fn vkCreateBuffer(
        device: VkDevice,
        pCreateInfo: *const VkBufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pBuffer: *mut VkBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html
    pub fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetBufferMemoryRequirements.html
    pub fn vkGetBufferMemoryRequirements(
        device: VkDevice,
        buffer: VkBuffer,
        pMemoryRequirements: *mut VkMemoryRequirements,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateMemory.html
    pub fn vkAllocateMemory(
        device: VkDevice,
        pAllocateInfo: *const VkMemoryAllocateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pMemory: *mut VkDeviceMemory,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkMapMemory.html
    pub fn vkMapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut c_void,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkUnmapMemory.html
    pub fn vkUnmapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkBindBufferMemory.html
    pub fn vkBindBufferMemory(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFlushMappedMemoryRanges.html
    pub fn vkFlushMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRange: *const VkMappedMemoryRange,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateCommandBuffers.html
    pub fn vkAllocateCommandBuffers(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffer: *mut VkCommandBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkBeginCommandBuffer.html
    pub fn vkBeginCommandBuffer(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEndCommandBuffer.html
    pub fn vkEndCommandBuffer(
        commandBuffer: VkCommandBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateFence.html
    pub fn vkCreateFence(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueSubmit.html
    pub fn vkQueueSubmit(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkWaitForFences.html
    pub fn vkWaitForFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyFence.html
    pub fn vkDestroyFence(
        device: VkDevice,
        fence: VkFence,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFreeCommandBuffers.html
    pub fn vkFreeCommandBuffers(
        device: VkDevice,
        commandPool: VkCommandPool,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdCopyBuffer.html
    pub fn vkCmdCopyBuffer(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDescriptorPool.html
    pub fn vkCreateDescriptorPool(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorPool: *mut VkDescriptorPool,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDescriptorSetLayout.html
    pub fn vkCreateDescriptorSetLayout(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSetLayout: *mut VkDescriptorSetLayout,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreatePipelineLayout.html
    pub fn vkCreatePipelineLayout(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateDescriptorSets.html
    pub fn vkAllocateDescriptorSets(
        device: VkDevice,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo,
        pDescriptorSets: *mut VkDescriptorSet,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkUpdateDescriptorSets.html
    pub fn vkUpdateDescriptorSets(
        device: VkDevice,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
        descriptorCopyCount: u32,
        pDescriptorCopies: *const VkCopyDescriptorSet,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreatePipelineCache.html
    pub fn vkCreatePipelineCache(
        device: VkDevice,
        pCreateInfo: *const VkPipelineCacheCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineCache: *mut VkPipelineCache,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateComputePipelines.html
    pub fn vkCreateComputePipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkComputePipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateShaderModule.html
    pub fn vkCreateShaderModule(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule,
    ) -> VkResult;
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdPipelineBarrier.html
    pub fn vkCmdPipelineBarrier(
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdBindPipeline.html
    pub fn vkCmdBindPipeline(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdBindDescriptorSets.html
    pub fn vkCmdBindDescriptorSets(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdDispatch.html
    pub fn vkCmdDispatch(
        commandBuffer: VkCommandBuffer,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkResetFences.html
    pub fn vkResetFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkInvalidateMappedMemoryRanges.html
    pub fn vkInvalidateMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueWaitIdle.html
    pub fn vkQueueWaitIdle(
        queue: VkQueue,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyBuffer.html
    pub fn vkDestroyBuffer(
        device: VkDevice,
        buffer: VkBuffer,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFreeMemory.html
    pub fn vkFreeMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyCommandPool.html
    pub fn vkDestroyCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyShaderModule.html
    pub fn vkDestroyShaderModule(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks,
    );
}