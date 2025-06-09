use std::{collections::HashMap, num::NonZero};

use crate::{BindGroupLayoutEntry, BindingType, RawBindGroupLayoutDescriptor, ShaderStages};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

impl BindGroupLayoutDescriptor {
    pub fn get_raw(&self) -> RawBindGroupLayoutDescriptor {
        RawBindGroupLayoutDescriptor {
            label: self.label.as_deref(),
            entries: &self.entries,
        }
    }
}

pub struct BindGroupLayoutEntriesBuilder {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
    default_visibility: ShaderStages,
    binding_to_entries: HashMap<u32, usize>,
}

impl BindGroupLayoutEntriesBuilder {
    pub fn new(default_visibility: ShaderStages) -> BindGroupLayoutEntriesBuilder {
        BindGroupLayoutEntriesBuilder {
            entries: vec![],
            default_visibility,
            binding_to_entries: HashMap::default(),
        }
    }

    pub fn add_bind_group_layout(
        &mut self,
        binding: u32,
        bind_group_layout: BindGroupLayoutEntryBuilder,
    ) {
        let bind_group_layout = bind_group_layout.build(binding, self.default_visibility);
        if let Some(index) = self.binding_to_entries.get(&binding) {
            self.entries[*index] = bind_group_layout;
        } else {
            let index = self.entries.len();
            self.entries.push(bind_group_layout);
            self.binding_to_entries.insert(binding, index);
        }
    }

    pub fn build(self) -> Vec<wgpu::BindGroupLayoutEntry> {
        self.entries
    }
}

#[derive(Clone, Copy)]
pub struct BindGroupLayoutEntryBuilder {
    ty: BindingType,
    visibility: Option<ShaderStages>,
    count: Option<NonZero<u32>>,
}

impl BindGroupLayoutEntryBuilder {
    pub fn visibility(mut self, visibility: ShaderStages) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn count(mut self, count: NonZero<u32>) -> Self {
        self.count = Some(count);
        self
    }

    pub fn build(&self, binding: u32, default_visibility: ShaderStages) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            ty: self.ty,
            visibility: self.visibility.unwrap_or(default_visibility),
            count: self.count,
        }
    }
}

pub trait IntoBindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder;
}

impl IntoBindGroupLayoutEntryBuilder for BindingType {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        BindGroupLayoutEntryBuilder {
            ty: self,
            visibility: None,
            count: None,
        }
    }
}

impl IntoBindGroupLayoutEntryBuilder for BindGroupLayoutEntry {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        BindGroupLayoutEntryBuilder {
            ty: self.ty,
            visibility: Some(self.visibility),
            count: self.count,
        }
    }
}

impl IntoBindGroupLayoutEntryBuilder for BindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        self
    }
}

pub mod binding_types {
    use core::num::NonZero;
    use encase::ShaderType;
    use wgpu::{BufferBindingType, SamplerBindingType, TextureSampleType, TextureViewDimension};
    use wgpu::{StorageTextureAccess, TextureFormat};

    use super::*;

    pub fn storage_buffer<T: ShaderType>(has_dynamic_offset: bool) -> BindGroupLayoutEntryBuilder {
        storage_buffer_sized(has_dynamic_offset, Some(T::min_size()))
    }

    pub fn storage_buffer_sized(
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u64>>,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset,
            min_binding_size,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn storage_buffer_read_only<T: ShaderType>(
        has_dynamic_offset: bool,
    ) -> BindGroupLayoutEntryBuilder {
        storage_buffer_read_only_sized(has_dynamic_offset, Some(T::min_size()))
    }

    pub fn storage_buffer_read_only_sized(
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u64>>,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset,
            min_binding_size,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn uniform_buffer<T: ShaderType>(has_dynamic_offset: bool) -> BindGroupLayoutEntryBuilder {
        uniform_buffer_sized(has_dynamic_offset, Some(T::min_size()))
    }

    pub fn uniform_buffer_sized(
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u64>>,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset,
            min_binding_size,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_1d(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D1,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_2d(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_2d_multisampled(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D2,
            multisampled: true,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_2d_array(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D2Array,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_2d_array_multisampled(
        sample_type: TextureSampleType,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D2Array,
            multisampled: true,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_depth_2d() -> BindGroupLayoutEntryBuilder {
        texture_2d(TextureSampleType::Depth).into_bind_group_layout_entry_builder()
    }

    pub fn texture_depth_2d_multisampled() -> BindGroupLayoutEntryBuilder {
        texture_2d_multisampled(TextureSampleType::Depth).into_bind_group_layout_entry_builder()
    }

    pub fn texture_cube(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::Cube,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_cube_multisampled(
        sample_type: TextureSampleType,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::Cube,
            multisampled: true,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_cube_array(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::CubeArray,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_cube_array_multisampled(
        sample_type: TextureSampleType,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::CubeArray,
            multisampled: true,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_3d(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D3,
            multisampled: false,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_3d_multisampled(sample_type: TextureSampleType) -> BindGroupLayoutEntryBuilder {
        BindingType::Texture {
            sample_type,
            view_dimension: TextureViewDimension::D3,
            multisampled: true,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn sampler(sampler_binding_type: SamplerBindingType) -> BindGroupLayoutEntryBuilder {
        BindingType::Sampler(sampler_binding_type).into_bind_group_layout_entry_builder()
    }

    pub fn texture_storage_2d(
        format: TextureFormat,
        access: StorageTextureAccess,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::StorageTexture {
            access,
            format,
            view_dimension: TextureViewDimension::D2,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_storage_2d_array(
        format: TextureFormat,
        access: StorageTextureAccess,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::StorageTexture {
            access,
            format,
            view_dimension: TextureViewDimension::D2Array,
        }
        .into_bind_group_layout_entry_builder()
    }

    pub fn texture_storage_3d(
        format: TextureFormat,
        access: StorageTextureAccess,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::StorageTexture {
            access,
            format,
            view_dimension: TextureViewDimension::D3,
        }
        .into_bind_group_layout_entry_builder()
    }
}
