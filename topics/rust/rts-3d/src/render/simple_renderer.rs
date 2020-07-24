use gl;
use log::{info,error};
use cgmath;
use std::mem::size_of;
use std::marker::PhantomData;
use std::ptr;
use std::ffi::c_void;

use super::gl_buffer::GlBuffer;
use super::gl_index_buffer::GlIndexBuffer;
use super::renderable::{Renderable,Vertex,Index};

const MAX_VERTICES: usize = 10_000;
const MAX_VBO_SIZE: usize = MAX_VERTICES * size_of::<Vertex>();
const MAX_IBO_SIZE: usize = 20_000;

pub struct SimpleRenderer<'a, T : Renderable> {
    vbo: GlBuffer,
    vbo_ptr: &'a mut [Vertex],   // Only available during begin-end window.
    ibo: GlIndexBuffer,
    ibo_ptr: &'a mut [Index],
    phantom: PhantomData<T>,
}

impl <'a, T: Renderable> SimpleRenderer<'a, T> {

//     pub fn new() -> SimpleRenderer<'a, T> {
//         SimpleRenderer::<T>{
//             vbo: GlBuffer::with_capacity(MAX_VBO_SIZE),
//             vbo_ptr: &mut [],
//             ibo: GlIndexBuffer::new(&[0; MAX_IBO_SIZE], 0),
//             ibo_ptr: &mut [],
//             phantom: PhantomData,
//         }
//     }

//     pub fn begin(&mut self) {
//         self.vbo.components = 0;
//         self.vbo_ptr = unsafe {
//             std::slice::from_raw_parts_mut(
//                 self.vbo.map_buffer(gl::WRITE_ONLY) as *mut Vertex,
//                 MAX_VBO_SIZE
//             )
//         };

//         self.ibo.components = 0;
//         self.ibo_ptr = unsafe {
//             std::slice::from_raw_parts_mut(
//                 self.ibo.map_buffer(gl::WRITE_ONLY) as *mut Index,
//                 MAX_IBO_SIZE
//             )
//         };
//     }
//     pub fn end(&mut self) {
//         self.vbo.unmap_buffer();
//     }

//     /// Copy to mem_buffer, will send to GPU in `present()` call.
//     pub fn submit(&mut self, renderable: &T) {
//         info!("Pre-vertices! {:?} {}", self.vbo_ptr, self.vbo_ptr.len());
//         renderable.get_vertices().iter().for_each(|vert| {
//             self.vbo_ptr[self.vbo.components] = *vert;
//             self.vbo.components += 1;
//         });

//         info!("Pre-indices!");
//         renderable.get_indices().iter().for_each(|index| {
//             self.ibo_ptr[self.ibo.components] = *index;
//             self.ibo.components += 1;
//         });
//         info!("Submitted")
//     }

//     pub fn present(&mut self) {
//         unsafe {
//             gl::DrawElements(gl::TRIANGLES, self.ibo.components as i32, gl::UNSIGNED_SHORT, ptr::null());
//         };
//     }
// }

// impl <'a, T: Renderable> Drop for SimpleRenderer<'a, T> {
//     fn drop(&mut self) {

//     }
}
