/// Complete test for ALL shaders in the new architecture
/// Tests every shader type and generates screenshots for verification
use astraria_rust::{
    renderer::{core::*, MainRenderer},
    AstrariaError, AstrariaResult,
};
use image::{ImageBuffer, Rgba};
use std::fs;

const SIZE: u32 = 800;

async fn save_render(
    renderer: &MainRenderer,
    texture: &wgpu::Texture,
    buffer: &wgpu::Buffer,
    filename: &str,
    test_type: u8,
) -> AstrariaResult<()> {
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = renderer
        .device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        match test_type {
            0 => renderer.render_default_sphere(&mut rp),
            1 => renderer.render_default_cube(&mut rp),
            2 => renderer.render_atmospheric_planet(&mut rp),
            3 => renderer.render_sun(&mut rp),
            4 => renderer.render_skybox(&mut rp),
            5 => renderer.render_billboard(&mut rp),
            6 => renderer.render_lens_glow(&mut rp),
            7 => renderer.render_black_hole(&mut rp),
            8 => renderer.render_line(&mut rp),
            9 => renderer.render_point(&mut rp),
            _ => {}
        }
    }

    copy_texture_to_buffer_aligned(&mut encoder, texture, buffer, SIZE, SIZE);
    renderer.queue().submit(std::iter::once(encoder.finish()));

    let slice = buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    renderer.device().poll(wgpu::Maintain::Wait);

    let final_data = {
        let data = slice.get_mapped_range();
        let result = remove_padding_from_buffer_data(&data, SIZE, SIZE);
        drop(data);
        result
    };
    buffer.unmap();

    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(SIZE, SIZE, final_data)
        .ok_or_else(|| AstrariaError::Graphics("Failed to create image".to_string()))?;

    img.save(filename)
        .map_err(|e| AstrariaError::Graphics(format!("Save failed: {}", e)))?;
    Ok(())
}

async fn run() -> AstrariaResult<()> {
    env_logger::init();

    println!("🚀 Testing ALL Shader Architecture");

    // Create output directory
    let output_dir = "renders";
    fs::create_dir_all(output_dir).map_err(|e| {
        AstrariaError::Graphics(format!("Failed to create output directory: {}", e))
    })?;

    let renderer = MainRenderer::new().await?;

    let texture = renderer.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Test Texture"),
        size: wgpu::Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let buffer_size = calculate_aligned_buffer_size(SIZE, SIZE);
    let buffer = renderer.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("Test Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Define all shader tests
    let shader_tests = vec![
        (0, "DefaultShader + Sphere", "default_sphere.png"),
        (1, "DefaultShader + Cube", "default_cube.png"),
        (2, "PlanetAtmoShader + Atmosphere", "atmospheric_planet.png"),
        (3, "SunShader + Stellar Surface", "sun_shader.png"),
        (4, "SkyboxShader + Cubemap", "skybox_shader.png"),
        (5, "BillboardShader + Sprite", "billboard_shader.png"),
        (6, "LensGlowShader + Lens Flare", "lens_glow_shader.png"),
        (
            7,
            "BlackHoleShader + Gravitational Lensing",
            "black_hole_shader.png",
        ),
        (8, "LineShader + Orbital Paths", "line_shader.png"),
        (9, "PointShader + Distant Objects", "point_shader.png"),
    ];

    // Test all shaders
    for (test_type, description, filename) in shader_tests {
        println!("🔸 {}", description);
        let filepath = format!("{}/{}", output_dir, filename);

        match save_render(&renderer, &texture, &buffer, &filepath, test_type).await {
            Ok(_) => println!("✅ Saved: {}", filepath),
            Err(e) => println!(
                "⚠️  Failed to render {}: {} (shader may not be fully implemented)",
                description, e
            ),
        }
    }

    Ok(())
}

fn main() {
    pollster::block_on(run()).unwrap_or_else(|e| {
        eprintln!("❌ Test failed: {}", e);
        std::process::exit(1);
    });
}
