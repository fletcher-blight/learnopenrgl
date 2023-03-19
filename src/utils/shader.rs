use rgl::prelude as rgl;

pub struct Shader {
    pub id: rgl::Shader,
}

impl Shader {
    pub fn new(source: &str, shader_type: rgl::ShaderType) -> Self {
        let id = rgl::create_shader(shader_type);
        let shader = Shader { id };

        rgl::shader_source(id, source);
        rgl::compile_shader(id);

        if !rgl::get_shader_compile_status(id) {
            let mut buffer = [0; 1024];
            let contents = rgl::get_shader_info_log(id, &mut buffer);
            let info_log = std::str::from_utf8(contents)
                .expect("Shader info log should be a valid utf8 string");
            panic!("Failed to compile shader: {info_log}");
        }

        shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        rgl::delete_shader(self.id)
    }
}

pub struct Program {
    pub id: rgl::Program,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Self {
        let id = rgl::create_program();
        let program = Program { id };

        for shader in shaders {
            rgl::attach_shader(id, shader.id);
        }
        rgl::link_program(id);

        if !rgl::get_program_link_status(id) {
            let mut buffer = [0; 1024];
            let contents = rgl::get_program_info_log(id, &mut buffer);
            let info_log = std::str::from_utf8(contents)
                .expect("Program info log should be a valid utf8 string");
            panic!("Failed to link program: {info_log}");
        }

        for shader in shaders {
            rgl::detach_shader(id, shader.id);
        }

        program
    }

    pub fn enable(&self) {
        rgl::use_program(self.id);
    }

    pub fn get_uniform_location(&self, name: &str) -> rgl::UniformLocation {
        let mut name_with_null_term = String::from(name);
        name_with_null_term.push('\0');
        rgl::get_uniform_location(
            self.id,
            std::ffi::CStr::from_bytes_with_nul(name_with_null_term.as_bytes())
                .expect("Requires cstr-able name"),
        )
    }

    pub fn set_uniform_i32(&self, name: &str, value: i32) {
        rgl::uniform_1i32(self.get_uniform_location(name), value)
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        rgl::uniform_1f32(self.get_uniform_location(name), value)
    }

    pub fn set_uniform_vec3(&self, name: &str, vec: [f32; 3]) {
        rgl::uniform_3f32v(self.get_uniform_location(name), &[vec])
    }

    pub fn set_uniform_mat4_flat(&self, name: &str, order: rgl::MatrixOrderMajor, mat: [f32; 16]) {
        rgl::uniform_matrix_4f32v_flat(self.get_uniform_location(name), order, &[mat])
    }

    pub fn set_uniform_mat4_multi(
        &self,
        name: &str,
        order: rgl::MatrixOrderMajor,
        mat: [[f32; 4]; 4],
    ) {
        rgl::uniform_matrix_4f32v_multi(self.get_uniform_location(name), order, &[mat])
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        rgl::delete_program(self.id)
    }
}
