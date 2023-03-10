use std::{cell::Cell, rc::Rc};

use bytemuck::Pod;
use glow::HasContext;

use super::{context::ContextShared, error::check_gl_error, BufferError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StaticDraw,
    StaticRead,
    DynamicDraw,
    DynamicRead,
}

impl BufferUsage {
    pub const fn to_gl(self) -> u32 {
        use BufferUsage::*;

        match self {
            StreamDraw => glow::STREAM_DRAW,
            StreamRead => glow::STREAM_READ,
            StaticDraw => glow::STATIC_DRAW,
            StaticRead => glow::STATIC_READ,
            DynamicDraw => glow::DYNAMIC_DRAW,
            DynamicRead => glow::DYNAMIC_READ,
        }
    }
}

pub(super) struct BufferShared {
    ctx: Rc<ContextShared>,
    id: glow::Buffer,
    usage: BufferUsage,
    len: Cell<usize>,
}

impl BufferShared {
    pub fn len(&self) -> usize {
        self.len.get()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Buffer {
    pub(super) shared: Rc<BufferShared>,
}

impl Buffer {
    pub(super) fn new<T: Pod>(
        ctx: Rc<ContextShared>,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, BufferError> {
        let gl = ctx.gl();
        let id = unsafe { gl.create_buffer() }.map_err(BufferError::ObjectCreation)?;

        let shared = Rc::new(BufferShared {
            ctx: ctx.clone(),
            id,
            usage,
            len: Cell::new(0),
        });

        let buffer = Buffer { shared };

        buffer.set(data);

        check_gl_error(gl).map_err(BufferError::Unexpected)?;

        Ok(buffer)
    }

    pub(super) fn context(&self) -> &ContextShared {
        &self.shared.ctx
    }

    pub fn id(&self) -> glow::Buffer {
        self.shared.id
    }

    pub fn usage(&self) -> BufferUsage {
        self.shared.usage
    }

    pub fn len(&self) -> usize {
        self.shared.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shared.is_empty()
    }

    pub fn set<T: Pod>(&self, data: &[T]) {
        let gl = self.shared.ctx.gl();
        let raw_data = bytemuck::cast_slice(data);

        // We can get away with always using `ARRAY_BUFFER` as the target here,
        // since the target does not carry any meaning for setting data. It is
        // just a binding point.
        let target = glow::ARRAY_BUFFER;

        unsafe {
            gl.bind_buffer(target, Some(self.shared.id));
            gl.buffer_data_u8_slice(target, raw_data, self.shared.usage.to_gl());

            // TODO: Could avoid unbinding here by using `ContextShared`.
            gl.bind_buffer(target, None);
        }

        self.shared.len.set(raw_data.len());
    }
}

impl Drop for BufferShared {
    fn drop(&mut self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.delete_buffer(self.id);
        }
    }
}
