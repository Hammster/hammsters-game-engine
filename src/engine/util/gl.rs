use gl;

pub fn init() {
    unsafe {
       // gl::ClearColor(0.0, 1.0, 0.0, 1.0); // Set background color to black and opaque
       // gl::ClearDepth(1.0);                // Set background depth to farthest
       // gl::Enable(gl::DEPTH_TEST);         // Enable depth testing for z-culling
       // gl::DepthFunc(gl::LEQUAL);          // Set the type of depth-test
    }
}

pub fn wireframe_mode () {
    unsafe {
        gl::PolygonMode(gl::FRONT, gl::LINE);
    }
}

pub fn redraw() {
    unsafe { 
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 18);
    }
}

