use qt_core::byte_array::ByteArray;
use std::ffi::c_void;
use glium::{SwapBuffersError, backend::Backend};

#[repr(C)]
pub struct QtBackend {
    context: *mut qt_gui::opengl_context::OpenGLContext,
}

impl QtBackend {
    /// The lifecycle of this object must match the `OpenGLContext` lifetime and thus is unsafe to create.
    pub unsafe fn new(context: *mut qt_gui::opengl_context::OpenGLContext) -> QtBackend {
        QtBackend { context }
    }
}

unsafe impl Backend for QtBackend {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        unsafe {
            (*self.context).swap_buffers((*self.context).surface());
        }
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        (*self.context)
            .get_proc_address(&ByteArray::new(()).append(&qt_core::string::String::from(symbol)))
            as *const c_void
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let size = unsafe {(*(*self.context).surface()).size()};
        (size.width() as u32, size.height() as u32)
    }

    fn is_current(&self) -> bool {
        qt_gui::opengl_context::OpenGLContext::current_context() == self.context
    }

    unsafe fn make_current(&self) {
        if !(*self.context).make_current((*self.context).surface()) {
            panic!("tried to make QtBackend context current, but it failed")
        }
    }
}
