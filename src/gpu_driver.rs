use std::slice;

use crate::{bitmap::Bitmap, platform::GPUDRIVER, rect::Rect};

pub struct RenderBuffer {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub has_stencil_buffer: bool,
    pub has_depth_buffer: bool,
}

impl From<ul_sys::ULRenderBuffer> for RenderBuffer {
    fn from(rb: ul_sys::ULRenderBuffer) -> Self {
        RenderBuffer {
            texture_id: rb.texture_id,
            width: rb.width,
            height: rb.height,
            has_stencil_buffer: rb.has_stencil_buffer,
            has_depth_buffer: rb.has_depth_buffer,
        }
    }
}
#[allow(non_camel_case_types)]
pub enum VertexBufferFormat {
    Format_2f_4ub_2f = ul_sys::ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f as isize,
    Format_2f_4ub_2f_2f_28f =
        ul_sys::ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f as isize,
}

impl TryFrom<ul_sys::ULVertexBufferFormat> for VertexBufferFormat {
    type Error = ();

    fn try_from(vbf: ul_sys::ULVertexBufferFormat) -> Result<Self, Self::Error> {
        match vbf {
            ul_sys::ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f)
            }
            ul_sys::ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f_2f_28f)
            }
            _ => Err(()),
        }
    }
}

pub struct VertexBuffer {
    pub format: VertexBufferFormat,
    pub buffer: Vec<u8>,
}

impl TryFrom<ul_sys::ULVertexBuffer> for VertexBuffer {
    type Error = ();

    fn try_from(vb: ul_sys::ULVertexBuffer) -> Result<Self, Self::Error> {
        let format = VertexBufferFormat::try_from(vb.format)?;
        let buffer = unsafe { slice::from_raw_parts(vb.data, vb.size as usize) };
        Ok(VertexBuffer {
            format,
            buffer: buffer.to_vec(),
        })
    }
}

pub struct IndexBuffer {
    pub buffer: Vec<u8>,
}

impl From<ul_sys::ULIndexBuffer> for IndexBuffer {
    fn from(vb: ul_sys::ULIndexBuffer) -> Self {
        let index_slice = unsafe { slice::from_raw_parts(vb.data, vb.size as usize) };
        IndexBuffer {
            buffer: index_slice.to_vec(),
        }
    }
}

pub enum GpuCommandType {
    DrawGeometry = ul_sys::ULCommandType_kCommandType_DrawGeometry as isize,
    ClearRenderBuffer = ul_sys::ULCommandType_kCommandType_ClearRenderBuffer as isize,
}

impl TryFrom<ul_sys::ULCommandType> for GpuCommandType {
    type Error = ();

    fn try_from(gct: ul_sys::ULCommandType) -> Result<Self, Self::Error> {
        match gct {
            ul_sys::ULCommandType_kCommandType_DrawGeometry => Ok(GpuCommandType::DrawGeometry),
            ul_sys::ULCommandType_kCommandType_ClearRenderBuffer => {
                Ok(GpuCommandType::ClearRenderBuffer)
            }
            _ => Err(()),
        }
    }
}

pub struct Matrix4x4 {
    pub data: [f32; 16],
}

pub struct Vec4 {
    pub data: [f32; 4],
}

// helper macro to convert arrays
macro_rules! from_ul_arr {
    ($struct:ident, $arr:expr, $to:ident : $from:ident) => {
        [
            $struct { $to: $arr[0].$from },
            $struct { $to: $arr[1].$from },
            $struct { $to: $arr[2].$from },
            $struct { $to: $arr[3].$from },
            $struct { $to: $arr[4].$from },
            $struct { $to: $arr[5].$from },
            $struct { $to: $arr[6].$from },
            $struct { $to: $arr[7].$from },
        ]
    };
}

pub enum ShaderType {
    Fill = ul_sys::ULShaderType_kShaderType_Fill as isize,
    FillPath = ul_sys::ULShaderType_kShaderType_FillPath as isize,
}

impl TryFrom<ul_sys::ULShaderType> for ShaderType {
    type Error = ();

    fn try_from(st: ul_sys::ULShaderType) -> Result<Self, Self::Error> {
        match st {
            ul_sys::ULShaderType_kShaderType_Fill => Ok(ShaderType::Fill),
            ul_sys::ULShaderType_kShaderType_FillPath => Ok(ShaderType::FillPath),
            _ => Err(()),
        }
    }
}

pub struct GpuState {
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub transform: Matrix4x4,
    pub enable_texturing: bool,
    pub enable_blend: bool,
    pub shader_type: ShaderType,
    pub render_buffer_id: u32,
    pub texture_1_id: Option<u32>,
    pub texture_2_id: Option<u32>,
    pub texture_3_id: Option<u32>,
    pub uniform_scalar: [f32; 8],
    pub uniform_vector: [Vec4; 8],
    pub clip_size: u8,
    pub clip: [Matrix4x4; 8],
    pub enable_scissor: bool,
    pub scissor_rect: Rect<i32>,
}

impl TryFrom<ul_sys::ULGPUState> for GpuState {
    type Error = ();

    fn try_from(gs: ul_sys::ULGPUState) -> Result<Self, Self::Error> {
        Ok(GpuState {
            viewport_width: gs.viewport_width,
            viewport_height: gs.viewport_height,
            transform: Matrix4x4 {
                data: gs.transform.data,
            },
            enable_texturing: gs.enable_texturing,
            enable_blend: gs.enable_blend,
            shader_type: ShaderType::try_from(gs.shader_type as u32)?,
            render_buffer_id: gs.render_buffer_id,
            texture_1_id: if gs.texture_1_id == 0 {
                None
            } else {
                Some(gs.texture_1_id)
            },
            texture_2_id: if gs.texture_2_id == 0 {
                None
            } else {
                Some(gs.texture_2_id)
            },
            texture_3_id: if gs.texture_3_id == 0 {
                None
            } else {
                Some(gs.texture_3_id)
            },
            uniform_scalar: gs.uniform_scalar,
            uniform_vector: from_ul_arr!(Vec4, gs.uniform_vector, data: value),
            clip_size: gs.clip_size,
            clip: from_ul_arr!(Matrix4x4, gs.clip, data: data),
            enable_scissor: gs.enable_scissor,
            scissor_rect: Rect::from(gs.scissor_rect),
        })
    }
}

pub struct GpuCommand {
    pub command_type: GpuCommandType,
    pub gpu_state: GpuState,
    pub geometry_id: u32,
    pub indices_count: u32,
    pub indices_offset: u32,
}

impl TryFrom<ul_sys::ULCommand> for GpuCommand {
    type Error = ();

    fn try_from(gc: ul_sys::ULCommand) -> Result<Self, Self::Error> {
        Ok(GpuCommand {
            command_type: GpuCommandType::try_from(gc.command_type as u32)?,
            gpu_state: GpuState::try_from(gc.gpu_state)?,
            geometry_id: gc.geometry_id,
            indices_count: gc.indices_count,
            indices_offset: gc.indices_offset,
        })
    }
}

pub trait GpuDriver {
    fn begin_synchronize(&mut self);
    fn end_synchronize(&mut self);
    fn next_texture_id(&mut self) -> u32;
    fn create_texture(&mut self, texture_id: u32, bitmap: &Bitmap);
    fn update_texture(&mut self, texture_id: u32, bitmap: &Bitmap);
    fn destroy_texture(&mut self, texture_id: u32);
    fn next_render_buffer_id(&mut self) -> u32;
    fn create_render_buffer(&mut self, render_buffer_id: u32, render_buffer: RenderBuffer);
    fn destroy_render_buffer(&mut self, render_buffer_id: u32);
    fn next_geometry_id(&mut self) -> u32;
    fn create_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: VertexBuffer,
        index_buffer: IndexBuffer,
    );
    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: VertexBuffer,
        index_buffer: IndexBuffer,
    );
    fn destroy_geometry(&mut self, geometry_id: u32);
    fn update_command_list(&mut self, command_list: Vec<GpuCommand>);
}

platform_set_interface_macro! {
    pub(crate) set_gpu_driver<GpuDriver>(gpu_driver -> GPUDRIVER) -> ulPlatformSetGPUDriver(ULGPUDriver) {
        begin_synchronize() -> () {}
        end_synchronize() -> () {}
        next_texture_id(() -> u32) -> () {}
        create_texture((texture_id: u32, ul_bitmap: ul_sys::ULBitmap)) -> ((texture_id: u32, bitmap: &Bitmap)) {
            let bitmap = Bitmap::from_raw(ul_bitmap);
            let bitmap = &bitmap;
        }
        update_texture((texture_id: u32, ul_bitmap: ul_sys::ULBitmap)) -> ((texture_id: u32, bitmap: &Bitmap)) {
            let bitmap = Bitmap::from_raw(ul_bitmap);
            let bitmap = &bitmap;
        }
        destroy_texture((texture_id: u32)) -> ((texture_id: u32)) {}
        next_render_buffer_id(() -> u32) -> () {}
        create_render_buffer((render_buffer_id: u32, ul_render_buffer: ul_sys::ULRenderBuffer))
            -> ((render_buffer_id: u32, render_buffer: RenderBuffer)) {
            let render_buffer = RenderBuffer::try_from(ul_render_buffer).unwrap();
        }
        destroy_render_buffer((render_buffer_id: u32)) -> ((render_buffer_id: u32)) {}
        next_geometry_id(() -> u32) -> () {}
        create_geometry((geometry_id: u32, ul_vertex_buffer: ul_sys::ULVertexBuffer,
            ul_index_buffer: ul_sys::ULIndexBuffer)) -> ((geometry_id: u32, vertex_buffer: VertexBuffer, index_buffer: IndexBuffer)) {
            let vertex_buffer = VertexBuffer::try_from(ul_vertex_buffer).unwrap();
            let index_buffer = IndexBuffer::from(ul_index_buffer);
        }
        update_geometry((geometry_id: u32, ul_vertex_buffer: ul_sys::ULVertexBuffer,
            ul_index_buffer: ul_sys::ULIndexBuffer)) -> ((geometry_id: u32, vertex_buffer: VertexBuffer, index_buffer: IndexBuffer)) {
            let vertex_buffer = VertexBuffer::try_from(ul_vertex_buffer).unwrap();
            let index_buffer = IndexBuffer::from(ul_index_buffer);
        }
        destroy_geometry((geometry_id: u32)) -> ((geometry_id: u32)) {}
        update_command_list((ul_command_list: ul_sys::ULCommandList)) -> ((commands_list: Vec<GpuCommand>)) {
            let commands_slice = slice::from_raw_parts(ul_command_list.commands, ul_command_list.size as usize);
            let commands_list = commands_slice.iter().map(|gc| GpuCommand::try_from(*gc).unwrap()).collect();
        }
    }
}
