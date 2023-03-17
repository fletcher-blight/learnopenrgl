use nalgebra_glm as glm;
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
        .window("LearnOpenGL: Camera", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
    rgl::shader_source(vertex_shader, include_str!("shader.vert"));
    rgl::compile_shader(vertex_shader);
    assert!(rgl::get_shader_compile_status(vertex_shader));

    let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
    rgl::shader_source(fragment_shader, include_str!("shader.frag"));
    rgl::compile_shader(fragment_shader);
    assert!(rgl::get_shader_compile_status(fragment_shader));

    let shader_program = rgl::create_program();
    rgl::attach_shader(shader_program, vertex_shader);
    rgl::attach_shader(shader_program, fragment_shader);
    rgl::link_program(shader_program);
    assert!(rgl::get_program_link_status(shader_program));
    rgl::use_program(shader_program);

    let texture = {
        let mut texture = rgl::Texture::default();
        rgl::gen_textures(std::slice::from_mut(&mut texture));

        rgl::active_texture(0);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
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

        let image = image::open("./assets/splatoon-face.png")?.flipv();
        rgl::tex_image_2d(
            rgl::TextureBinding2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGBA,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(image.as_bytes()),
        );
        let loc = rgl::get_uniform_location(
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"tex1\0")?,
        );
        rgl::uniform_1i32(loc, 0);
        texture
    };

    let mut vao = Default::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = Default::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        &[
            // front
            [-0.5f32, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // left
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // top
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            // right
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            // bottom
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            // back
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
        ],
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Triple,
        rgl::VertexAttribFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 3) as u64,
        0,
    );
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer(0));

    let model = rgl::get_uniform_location(
        shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
    );
    let view = rgl::get_uniform_location(
        shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
    );
    let projection = rgl::get_uniform_location(
        shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
    );

    // ============================================================================================

    let mut camera_position = glm::vec3(0.0, 0.0, 3.0f32);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    const MOUSE_SENSITIVITY: f32 = 0.1;
    let mut yaw = -90.0f32;
    let mut pitch = 0.0f32;
    let mut direction_request = [0f32; 4];

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_frame_time_point = std::time::Instant::now();
    'main: loop {
        let frame_time_point = std::time::Instant::now();
        let frame_duration = frame_time_point - last_frame_time_point;
        last_frame_time_point = frame_time_point;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W => direction_request[0] = 0.0,
                        sdl2::keyboard::Keycode::S => direction_request[1] = 0.0,
                        sdl2::keyboard::Keycode::A => direction_request[2] = 0.0,
                        sdl2::keyboard::Keycode::D => direction_request[3] = 0.0,
                        _ => {}
                    };
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W => direction_request[0] = 1.0,
                        sdl2::keyboard::Keycode::S => direction_request[1] = 1.0,
                        sdl2::keyboard::Keycode::A => direction_request[2] = 1.0,
                        sdl2::keyboard::Keycode::D => direction_request[3] = 1.0,
                        _ => {}
                    };
                }
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    yaw += MOUSE_SENSITIVITY * (xrel as f32);
                    pitch -= MOUSE_SENSITIVITY * (yrel as f32);
                    pitch = pitch.clamp(-89.0, 89.0);
                }
                _ => {}
            }
        }

        let camera_front = glm::normalize(&glm::vec3(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos(),
        ));

        {
            let camera_right = glm::normalize(&glm::cross(&camera_front, &camera_up));
            let camera_position_diff = (camera_right
                * (direction_request[3] - direction_request[2]))
                + (camera_front * (direction_request[0] - direction_request[1]));

            camera_position += 2.5 * frame_duration.as_secs_f32() * camera_position_diff;
        }

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
        rgl::use_program(shader_program);

        rgl::uniform_matrix_4f32v(model, rgl::MatrixOrderMajor::Row, &[from_glm(&glm::one())]);
        rgl::uniform_matrix_4f32v(
            view,
            rgl::MatrixOrderMajor::Row,
            &[from_glm(&glm::look_at(
                &camera_position,
                &(camera_position + camera_front),
                &camera_up,
            ))],
        );
        rgl::uniform_matrix_4f32v(
            projection,
            rgl::MatrixOrderMajor::Row,
            &[from_glm(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        );

        rgl::active_texture(0);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
        rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);

        window.gl_swap_window();
    }

    Ok(())
}

fn from_glm(mat: &glm::Mat4) -> [f32; 16] {
    #[rustfmt::skip]
    return [
        mat.m11, mat.m12, mat.m13, mat.m14,
        mat.m21, mat.m22, mat.m23, mat.m24,
        mat.m31, mat.m32, mat.m33, mat.m34,
        mat.m41, mat.m42, mat.m43, mat.m44,
    ];
}
