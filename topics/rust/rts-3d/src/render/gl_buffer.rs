use gl;
use std::mem::size_of;
use std::ffi::c_void;
use std::marker::PhantomData;

pub struct GlBuffer<T> {
    phantom: PhantomData<T>,
    buffer_id: gl::types::GLuint,
}

impl <T> GlBuffer<T> {
    pub fn new(data: &[T]) -> GlBuffer<T> {
        let mut res = GlBuffer::<T>{
            buffer_id: 0,
            phantom: PhantomData,
        };

        res.init(data);

        res
    }

    /// Assumption: this _shouldn't_ modify the data passed, just cpy it to the buffer. 
    /// So it _should_ be safe, but I may be wrong.
    fn init(&mut self, data: &[T]) {
        unsafe {
            gl::GenBuffers(1, &mut self.buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);

            gl::BufferData(
                gl::ARRAY_BUFFER, 
                data.len() as isize * size_of::<T>() as isize, 
                data.as_ptr() as *mut c_void,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl <T> Drop for GlBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            // Silently ignores dropping bound buffers, so don't worry about it
            gl::DeleteBuffers(1, &mut self.buffer_id);
        }
    }
}
