use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Clone)]
pub struct RenderingContext {
	document: Document,
	canvas: HtmlCanvasElement,
	context: WebGl2RenderingContext,
	vert_shader: WebGlShader,
	frag_shader: WebGlShader,
	program: WebGlProgram,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(unused_variables)]
struct Vertex {
	pos: [f32; 3],
	line_start: [f32; 2],
	line_end: [f32; 2],
}

fn create_vertices(lines: &[(f32, f32, f32, f32)], padding: f32) -> (Vec<Vertex>, Vec<(glam::Vec2, glam::Vec2)>) {
	let mut positions = Vec::with_capacity(lines.len() * 4);
	let mut line_attrib = Vec::with_capacity(lines.len() * 4);
	let mut index_data = Vec::with_capacity(lines.len() * 6);
	for line in lines {
		use glam::Vec2;
		let a: Vec2 = (line.0, line.1).into();
		let b: Vec2 = (line.2, line.3).into();

		let v = (a - b).normalize_or_zero() * std::f32::consts::SQRT_2 * padding;
		let pv = v.perp();
		let a1 = a + v + pv;
		let a2 = a + v - pv;
		let b1 = b - v - pv;
		let b2 = b - v + pv;

		for index in &[0, 1, 2, 2, 3, 0] {
			index_data.push(positions.len() as u16 + index);
		}
		for point in &[a1, a2, b1, b1, b2, a1] {
			positions.push(Vertex {
				pos: [point.x, point.y, 0.],
				line_start: a.into(),
				line_end: b.into(),
			});
			line_attrib.push((a, b));
		}
	}
	(positions.to_vec(), line_attrib.to_vec())
}

impl RenderingContext {
	pub fn new() -> Result<Self, JsValue> {
		let document = web_sys::window().unwrap().document().unwrap();
		let canvas = document.query_selector(".rendering-canvas").unwrap().unwrap();
		let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

		let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;

		let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, include_str!("../shaders/shader.vert"))?;

		let frag_shader = compile_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, include_str!("../shaders/shader.frag"))?;
		let program = link_program(&context, &vert_shader, &frag_shader)?;
		context.use_program(Some(&program));
		Ok(Self {
			document,
			canvas,
			context,
			vert_shader,
			frag_shader,
			program,
		})
	}

	pub fn draw(&mut self) -> Result<(), JsValue> {
		//let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
		let (vertex_data, index_data) = create_vertices(&[(-0.5, -0.5, 0.5, 0.5), (-0.5, 0.5, 0.5, -0.5), (-0.5, -0.5, 0.5, -0.5), (-0.5, 0.5, 0.5, 0.5)], 0.15);

		log::debug!("vertices: {vertex_data:?}");
		let vertices: &[f32] = unsafe { std::slice::from_raw_parts(vertex_data.as_ptr() as *const f32, vertex_data.len() * std::mem::size_of::<Vertex>() / std::mem::size_of::<f32>()) };
		log::debug!("vertices: {vertices:?}");

		let position_attribute_location = self.context.get_attrib_location(&self.program, "position");
		let line_attribute_location = 1; //self.context.get_attrib_location(&self.program, "line");
		let buffer = self.context.create_buffer().ok_or("Failed to create buffer")?;
		self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

		// Note that `Float32Array::view` is somewhat dangerous (hence the
		// `unsafe`!). This is creating a raw view into our module's
		// `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
		// (aka do a memory allocation in Rust) it'll cause the buffer to change,
		// causing the `Float32Array` to be invalid.
		//
		// As a result, after `Float32Array::view` we have to be very careful not to
		// do any memory allocations before it's dropped.
		//let vertices = std::mem::transmute(&vertices[..]);
		log::debug!("vertices: {vertices:?}");
		let positions_array_buf_view = js_sys::Float32Array::new_with_length(vertices.len() as u32);
		positions_array_buf_view.copy_from(vertices);

		self.context
			.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &positions_array_buf_view, WebGl2RenderingContext::STATIC_DRAW);

		let vao = self.context.create_vertex_array().ok_or("Could not create vertex array object")?;
		self.context.bind_vertex_array(Some(&vao));
		log::debug!("{position_attribute_location:?}");
		log::debug!("{line_attribute_location:?}");

		self.context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 28, 0);
		self.context.enable_vertex_attrib_array(position_attribute_location as u32);
		self.context.vertex_attrib_pointer_with_i32(1, 4, WebGl2RenderingContext::FLOAT, false, 28, 12);
		self.context.enable_vertex_attrib_array(line_attribute_location as u32);

		let vert_count = (vertices.len() / 7) as i32;
		log::debug!("vert count {vert_count}");
		draw(&self.context, vert_count);

		Ok(())
	}
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
	context.clear_color(0.6, 0.5, 1.0, 1.0);
	context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
	context.enable(WebGl2RenderingContext::DEPTH_TEST);
	context.depth_func(WebGl2RenderingContext::LESS);

	context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
	let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;
	context.shader_source(&shader, source);
	context.compile_shader(&shader);

	if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
		Ok(shader)
	} else {
		Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
	}
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
	let program = context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

	context.attach_shader(&program, vert_shader);
	context.attach_shader(&program, frag_shader);
	context.link_program(&program);

	if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
		Ok(program)
	} else {
		Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
	}
}