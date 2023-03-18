use image::{ColorType, GenericImageView};
use nalgebra_glm as glm;
use rand::Rng;
use rgl::prelude as rgl;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    sdl.mouse().set_relative_mouse_mode(true);
    let window = video
        .window("LearnOpenGL: Instancing", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let asteroid_model = WavefrontObjModel::new("./assets/rock/rock.obj")?;
    let planet_model = WavefrontObjModel::new("./assets/planet/planet.obj")?;

    let asteroid_shader_program = Program::new(&[
        Shader::new(include_str!("asteroid.vert"), rgl::ShaderType::Vertex)?,
        Shader::new(include_str!("asteroid.frag"), rgl::ShaderType::Fragment)?,
    ])?;
    let planet_shader_program = Program::new(&[
        Shader::new(include_str!("planet.vert"), rgl::ShaderType::Vertex)?,
        Shader::new(include_str!("planet.frag"), rgl::ShaderType::Fragment)?,
    ])?;

    const NUM_ASTEROIDS: u64 = 100000;

    let mut rng = rand::thread_rng();
    let asteroid_model_mats: Vec<f32> = (0..NUM_ASTEROIDS)
        .flat_map(|_| {
            let scale_factor = 0.1 * rng.gen::<f32>();
            let angle: f32 = 360.0 * rng.gen::<f32>();
            let tilt_angle: f32 = 360.0 * rng.gen::<f32>();
            let tilt_axis = glm::vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());

            let horizontal_position = glm::rotate_y_vec3(&glm::vec3(100.0, 0.0, 0.0), angle);

            let fuzz_dir = glm::vec3(angle.to_radians().sin(), 0.0, angle.to_radians().cos());
            let fuzz_offset = fuzz_dir * rng.gen::<f32>() * 10.0;
            let vertical_fuzz = glm::vec3(
                0.0,
                (2.0 * rng.gen::<f32>()) - 1.0 + (0.1 * rng.gen::<f32>()),
                0.0,
            );

            let position_model = glm::translate(
                &glm::one(),
                &(horizontal_position + fuzz_offset + vertical_fuzz),
            );
            let scale_model = glm::scale(&position_model, &glm::make_vec3(&[scale_factor; 3]));
            let rotate_model = glm::rotate(&scale_model, tilt_angle.to_radians(), &tilt_axis);

            Vec::from(rotate_model.as_slice())
        })
        .collect();

    let mut instance_buffer = rgl::Buffer::default();
    rgl::gen_buffers(std::slice::from_mut(&mut instance_buffer));
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, instance_buffer);
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        asteroid_model_mats.as_slice(),
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );

    for mesh in &asteroid_model.meshes {
        rgl::bind_vertex_array(mesh.vao);
        rgl::bind_buffer(rgl::BufferBindingTarget::Array, instance_buffer);
        for attribute in 3..=6 {
            rgl::enable_vertex_attrib_array(attribute);
            rgl::vertex_attrib_float_pointer(
                attribute,
                rgl::VertexAttribSize::Quad,
                rgl::VertexAttribFloatType::F32,
                false,
                16 * std::mem::size_of::<f32>() as u64,
                (attribute - 3) as u64 * (4 * std::mem::size_of::<f32>()) as u64,
            );
            rgl::vertex_attrib_divisor(attribute, 1);
        }

        rgl::bind_vertex_array(rgl::VertexArray::default());
    }
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer::default());

    let mut camera = Camera::new();
    camera.position = [50.0, 50.0, 50.0];

    rgl::clear_colour(0.1, 0.1, 0.1, 0.1);

    // ============================================================================================

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_frame_time_point = std::time::Instant::now();
    let start_time_point = last_frame_time_point;
    'main: loop {
        let frame_time_point = std::time::Instant::now();
        let frame_duration = frame_time_point - last_frame_time_point;
        let total_duration = frame_time_point - start_time_point;
        last_frame_time_point = frame_time_point;

        let total_secs_passed = total_duration.as_secs_f32();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                event => process_sdl_event(&mut camera, event),
            }
        }

        camera.update_position(frame_duration.as_secs_f32(), 50.0);

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        let camera_view = camera.calculate_view();
        let camera_projection = camera.calculate_projection(window.size(), 45.0, 0.1, 1000.0);

        {
            planet_shader_program.enable();
            planet_shader_program.set_uniform_mat4(
                planet_shader_program.find_uniform("model")?,
                from_glm_mat4(&glm::scale(&glm::one(), &glm::vec3(5.0, 5.0, 5.0))),
            )?;
            planet_shader_program
                .set_uniform_mat4(planet_shader_program.find_uniform("view")?, camera_view)?;
            planet_shader_program.set_uniform_mat4(
                planet_shader_program.find_uniform("projection")?,
                camera_projection,
            )?;
            planet_model.draw(&planet_shader_program, 1)?;
        }
        {
            asteroid_shader_program.enable();
            asteroid_shader_program.set_uniform_mat4(
                asteroid_shader_program.find_uniform("rotate_model")?,
                from_glm_mat4(&glm::rotate(
                    &glm::one(),
                    (5.0 * total_secs_passed).to_radians(),
                    &glm::vec3(0.0, 1.0, 0.0),
                )),
            )?;
            asteroid_shader_program
                .set_uniform_mat4(asteroid_shader_program.find_uniform("view")?, camera_view)?;
            asteroid_shader_program.set_uniform_mat4(
                asteroid_shader_program.find_uniform("projection")?,
                camera_projection,
            )?;
            asteroid_model.draw(&asteroid_shader_program, NUM_ASTEROIDS)?;
        }

        window.gl_swap_window();
    }

    Ok(())
}

// ============================================================================================

struct Shader(rgl::Shader);
impl Drop for Shader {
    fn drop(&mut self) {
        rgl::delete_shader(self.0);
    }
}
impl Shader {
    fn new(source: &str, shader_type: rgl::ShaderType) -> anyhow::Result<Self> {
        let shader = Shader(rgl::create_shader(shader_type));
        rgl::shader_source(shader.0, source);
        rgl::compile_shader(shader.0);

        if rgl::get_shader_compile_status(shader.0) {
            Ok(shader)
        } else {
            let mut buffer = [0; 1024];
            let buffer = rgl::get_shader_info_log(shader.0, &mut buffer);
            let info_log: String = String::from_utf8_lossy(buffer).into_owned();
            Err(anyhow::Error::msg(format!(
                "{shader_type:?} Shader Compilation Failed: {info_log}"
            )))
        }
    }
}

struct Program(rgl::Program);
impl Drop for Program {
    fn drop(&mut self) {
        rgl::delete_program(self.0);
    }
}
impl Program {
    fn new(shaders: &[Shader]) -> anyhow::Result<Self> {
        let shader_program = Program(rgl::create_program());

        for shader in shaders {
            rgl::attach_shader(shader_program.0, shader.0);
        }

        rgl::link_program(shader_program.0);

        for shader in shaders {
            rgl::detach_shader(shader_program.0, shader.0);
        }

        if rgl::get_program_link_status(shader_program.0) {
            Ok(shader_program)
        } else {
            let mut buffer = [0; 1024];
            let buffer = rgl::get_program_info_log(shader_program.0, &mut buffer);
            let info_log: String = String::from_utf8_lossy(buffer).into_owned();
            Err(anyhow::Error::msg(format!(
                "Shader Program Link Failed: {info_log}"
            )))
        }
    }

    fn enable(&self) {
        rgl::use_program(self.0);
    }

    fn find_uniform(&self, name: &str) -> anyhow::Result<rgl::UniformLocation> {
        self.enable();
        let name = std::ffi::CString::new(name)?;
        let location = rgl::get_uniform_location(self.0, &name);
        Ok(location)
    }

    fn set_uniform_mat4(
        &self,
        location: rgl::UniformLocation,
        value: [f32; 16],
    ) -> anyhow::Result<()> {
        self.enable();
        rgl::uniform_matrix_4f32v_flat(location, rgl::MatrixOrderMajor::Row, &[value]);
        Ok(())
    }
}

struct Texture(rgl::Texture);
impl Drop for Texture {
    fn drop(&mut self) {
        rgl::delete_textures(&[self.0]);
    }
}
impl Texture {
    fn new(data: &[u8], format: rgl::TextureFormat, size: (u32, u32)) -> anyhow::Result<Self> {
        let mut texture = rgl::Texture::default();
        rgl::gen_textures(std::slice::from_mut(&mut texture));
        let texture = Texture(texture);

        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture.0);
        rgl::texture_target_wrap_s(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapMode::Repeat,
        );
        rgl::texture_target_wrap_t(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapMode::Repeat,
        );
        rgl::texture_target_min_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMinFilter::Linear,
        );
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        );

        rgl::tex_image_2d(
            rgl::TextureBinding2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            size.0,
            size.1,
            format,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(data),
        );
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, rgl::Texture::default());

        Ok(texture)
    }
}

struct InstancedIndexedVertexUVMesh {
    vao: rgl::VertexArray,
    vbo: rgl::Buffer,
    ebo: rgl::Buffer,
    diffuse: rgl::Texture,
    indices_count: u64,
}
impl Drop for InstancedIndexedVertexUVMesh {
    fn drop(&mut self) {
        rgl::delete_vertex_arrays(&[self.vao]);
        rgl::delete_buffers(&[self.vbo, self.ebo]);
    }
}
impl InstancedIndexedVertexUVMesh {
    fn new(vertices: &[f32], indices: &[u32], diffuse: rgl::Texture) -> anyhow::Result<Self> {
        let mut vao = rgl::VertexArray::default();
        let mut buffers = [rgl::Buffer::default(); 2];

        rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));
        rgl::gen_buffers(&mut buffers);
        let [vbo, ebo] = buffers;

        let mesh = InstancedIndexedVertexUVMesh {
            vao,
            vbo,
            ebo,
            diffuse,
            indices_count: indices.len() as u64,
        };

        rgl::bind_vertex_array(mesh.vao);
        rgl::bind_buffer(rgl::BufferBindingTarget::Array, mesh.vbo);
        rgl::bind_buffer(rgl::BufferBindingTarget::ElementArray, mesh.ebo);

        rgl::buffer_data(
            rgl::BufferBindingTarget::Array,
            vertices,
            rgl::BufferUsageFrequency::Static,
            rgl::BufferUsageNature::Draw,
        );
        rgl::buffer_data(
            rgl::BufferBindingTarget::ElementArray,
            indices,
            rgl::BufferUsageFrequency::Static,
            rgl::BufferUsageNature::Draw,
        );

        let position_size = std::mem::size_of::<f32>() * 3;
        let uv_size = std::mem::size_of::<f32>() * 2;
        let stride = position_size + uv_size;

        // position triple
        rgl::enable_vertex_attrib_array(0);
        rgl::vertex_attrib_float_pointer(
            0,
            rgl::VertexAttribSize::Triple,
            rgl::VertexAttribFloatType::F32,
            false,
            stride as u64,
            0,
        );

        // uv duple
        rgl::enable_vertex_attrib_array(1);
        rgl::vertex_attrib_float_pointer(
            1,
            rgl::VertexAttribSize::Double,
            rgl::VertexAttribFloatType::F32,
            false,
            stride as u64,
            position_size as u64,
        );

        rgl::bind_vertex_array(rgl::VertexArray::default());
        rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer::default());
        rgl::bind_buffer(
            rgl::BufferBindingTarget::ElementArray,
            rgl::Buffer::default(),
        );

        Ok(mesh)
    }
}

struct WavefrontObjModel {
    _materials: Vec<Texture>,
    meshes: Vec<InstancedIndexedVertexUVMesh>,
}
impl WavefrontObjModel {
    fn new(file_path: &str) -> anyhow::Result<Self> {
        let scene = russimp::scene::Scene::from_file(
            file_path,
            vec![
                russimp::scene::PostProcess::Triangulate,
                russimp::scene::PostProcess::FlipUVs,
            ],
        )?;

        let file_path_directory = std::path::Path::parent(std::path::Path::new(file_path)).unwrap();

        let mut material_map = std::collections::HashMap::new();

        for (i, material) in scene.materials.iter().enumerate() {
            let diffuse_material = material.properties.iter().find(|property| {
                property.semantic == russimp::material::TextureType::Diffuse
                    || property.semantic == russimp::material::TextureType::Height
            });

            if diffuse_material.is_none() {
                continue;
            }

            use russimp::material::PropertyTypeInfo;
            match &diffuse_material.unwrap().data {
                PropertyTypeInfo::Buffer(_) => unimplemented!(),
                PropertyTypeInfo::IntegerArray(_) => unimplemented!(),
                PropertyTypeInfo::FloatArray(_) => unimplemented!(),
                PropertyTypeInfo::String(filename) => {
                    println!("Loading material from file: {filename} ...");
                    let image = image::open(file_path_directory.join(filename))?.flipv();
                    let format = match image.color() {
                        ColorType::Rgb8 => rgl::TextureFormat::RGB,
                        ColorType::Rgba8 => rgl::TextureFormat::RGBA,
                        _ => unimplemented!("Unhandled image colour format type"),
                    };
                    let diffuse_texture =
                        Texture::new(image.as_bytes(), format, image.dimensions())?;
                    material_map.insert(i, diffuse_texture);
                    println!("Loading material from file: {filename} ... complete");
                }
            }
        }

        let mut meshes = Vec::new();
        for mesh in scene.meshes {
            let mut indices = Vec::new();
            for face in mesh.faces {
                indices.append(&mut face.0.clone());
            }

            let positions = mesh.vertices;
            let uvs = mesh.texture_coords.first().unwrap().as_ref().unwrap();

            let vertices: Vec<_> = itertools::izip!(positions, uvs)
                .flat_map(|(position, uv)| [position.x, position.y, position.z, uv.x, uv.y])
                .collect();

            let diffuse = material_map
                .get(&(mesh.material_index as usize))
                .ok_or_else(|| {
                    anyhow::Error::msg(format!("Unknown material index: {}", mesh.material_index))
                })?;

            meshes.push(InstancedIndexedVertexUVMesh::new(
                &vertices, &indices, diffuse.0,
            )?);
        }

        Ok(WavefrontObjModel {
            _materials: material_map.into_values().collect(),
            meshes,
        })
    }

    fn draw(&self, shader_program: &Program, instance_count: u64) -> anyhow::Result<()> {
        shader_program.enable();

        rgl::uniform_1i32(shader_program.find_uniform("material.diffuse")?, 0);

        for mesh in &self.meshes {
            rgl::active_texture(0);
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, mesh.diffuse);

            rgl::bind_vertex_array(mesh.vao);
            rgl::draw_elements_instanced(
                rgl::DrawMode::Triangles,
                mesh.indices_count,
                rgl::DrawIndexType::U32,
                0,
                instance_count,
            );
            rgl::bind_vertex_array(rgl::VertexArray::default());
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum MovementDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
impl MovementDirection {
    fn as_request_index(&self) -> usize {
        match self {
            MovementDirection::Forward => 0,
            MovementDirection::Backward => 1,
            MovementDirection::Left => 2,
            MovementDirection::Right => 3,
            MovementDirection::Up => 4,
            MovementDirection::Down => 5,
        }
    }
}

struct Camera {
    position: [f32; 3],
    yaw: f32,
    pitch: f32,
    movement_request: [f32; 6],
}
impl Camera {
    fn new() -> Self {
        Camera {
            position: [0.0, 0.0, 0.0],
            yaw: -90.0,
            pitch: 0.0,
            movement_request: [0.0; 6],
        }
    }

    fn set_move_direction(&mut self, direction: MovementDirection, amount: f32) {
        let index = direction.as_request_index();
        self.movement_request[index] = amount;
    }

    fn move_view(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset;
        self.pitch = (self.pitch + y_offset).clamp(-89.0, 89.0);
    }

    fn update_position(&mut self, secs_since_last_update: f32, sensitivity: f32) {
        let front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        let up = glm::vec3(0.0, 1.0, 0.0);

        let right = glm::normalize(&glm::cross(&front, &up));

        let calculate_factor = |start: MovementDirection, finish: MovementDirection| {
            self.movement_request[start.as_request_index()]
                - self.movement_request[finish.as_request_index()]
        };

        let mut position_diff = glm::Vec3::default();
        position_diff +=
            right * calculate_factor(MovementDirection::Right, MovementDirection::Left);
        position_diff +=
            front * calculate_factor(MovementDirection::Forward, MovementDirection::Backward);
        position_diff += up * calculate_factor(MovementDirection::Up, MovementDirection::Down);

        let position_diff = sensitivity * secs_since_last_update * position_diff;
        let position = glm::make_vec3(&self.position) + position_diff;
        self.position = [position[0], position[1], position[2]];
    }

    fn calculate_view(&self) -> [f32; 16] {
        let position = glm::make_vec3(&self.position);
        let front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        let up = glm::vec3(0.0, 1.0, 0.0);

        let transform = glm::look_at(&position, &(position + front), &up);
        from_glm_mat4(&transform)
    }

    fn calculate_projection(
        &self,
        window_size: (u32, u32),
        fov_deg: f32,
        near: f32,
        far: f32,
    ) -> [f32; 16] {
        let fov = fov_deg.to_radians();
        let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;

        let transform = glm::perspective(aspect_ratio, fov, near, far);
        from_glm_mat4(&transform)
    }
}

fn process_sdl_event(camera: &mut Camera, event: sdl2::event::Event) {
    fn try_keycode_as_direction(keycode: sdl2::keyboard::Keycode) -> Option<MovementDirection> {
        match keycode {
            sdl2::keyboard::Keycode::W => Some(MovementDirection::Forward),
            sdl2::keyboard::Keycode::S => Some(MovementDirection::Backward),
            sdl2::keyboard::Keycode::A => Some(MovementDirection::Left),
            sdl2::keyboard::Keycode::D => Some(MovementDirection::Right),
            sdl2::keyboard::Keycode::LShift => Some(MovementDirection::Up),
            sdl2::keyboard::Keycode::LCtrl => Some(MovementDirection::Down),
            _ => None,
        }
    }

    match event {
        sdl2::event::Event::KeyUp {
            keycode: Some(keycode),
            ..
        } => {
            if let Some(direction) = try_keycode_as_direction(keycode) {
                camera.set_move_direction(direction, 0.0);
            }
        }
        sdl2::event::Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            if let Some(direction) = try_keycode_as_direction(keycode) {
                camera.set_move_direction(direction, 1.0);
            }
        }
        sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
            const MOUSE_SENSITIVITY: f32 = 0.1;
            let x_offset = MOUSE_SENSITIVITY * (xrel as f32);
            let y_offset = -MOUSE_SENSITIVITY * (yrel as f32);
            camera.move_view(x_offset, y_offset);
        }
        _ => {}
    }
}

fn from_glm_mat4(mat: &nalgebra_glm::Mat4) -> [f32; 16] {
    #[rustfmt::skip]
    return [
        mat.m11, mat.m12, mat.m13, mat.m14,
        mat.m21, mat.m22, mat.m23, mat.m24,
        mat.m31, mat.m32, mat.m33, mat.m34,
        mat.m41, mat.m42, mat.m43, mat.m44,
    ];
}
