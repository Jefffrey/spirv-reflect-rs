use types::{ReflectResourceType, ReflectTypeDescription, ReflectBindingArrayTraits, ReflectBlockVariable, ReflectImageTraits};

#[derive(Debug, Copy, Clone)]
pub enum ReflectDescriptorType {
    Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
}

impl Default for ReflectDescriptorType {
    fn default() -> Self {
        ReflectDescriptorType::Undefined
    }
}

pub type ReflectOrdinalBinding = u32;
pub type ReflectOrdinalSet = u32;
pub type ReflectDescriptorBindingSet = (ReflectOrdinalBinding, ReflectOrdinalSet);

#[derive(Default, Debug, Clone)]
pub struct ReflectDescriptorBinding {
    pub spirv_id: u32,
    pub name: String,
    pub binding: u32,
    pub input_attachment_index: u32,
    pub set: u32,
    pub descriptor_type: ReflectDescriptorType,
    pub resource_type: ReflectResourceType,
    pub image: ReflectImageTraits,
    pub block: ReflectBlockVariable,
    pub array: ReflectBindingArrayTraits,
    pub count: u32,
    pub uav_counter_id: u32,
    pub uav_counter_binding: Option<Box<ReflectDescriptorBinding>>,
    pub type_description: Option<ReflectTypeDescription>,
    pub word_offset: ReflectDescriptorBindingSet,
}

#[derive(Default, Debug, Clone)]
pub struct ReflectDescriptorSet {
    pub set: u32,
    pub bindings: Vec<ReflectDescriptorBinding>,
}