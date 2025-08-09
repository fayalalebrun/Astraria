/// Simplified occlusion proxy shader using ONLY generated default shader bindings
/// This avoids all struct alignment issues by reusing existing generated types
use wgpu;

/// Minimal occlusion proxy shader that reuses default shader's generated bind groups
pub struct OcclusionProxyShader {
    /// Render pipeline for invisible proxy geometry
    pub render_pipeline: wgpu::RenderPipeline,
}

impl OcclusionProxyShader {
    /// Create a new occlusion proxy shader using default shader's bind group layouts
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Use extremely simple inline shader that only needs MVP matrix (group 0)
        let shader_source = r#"
            // Minimal occlusion proxy shader - only uses MVP matrix
            
            struct StandardMVPUniform {
                mvp_matrix: mat4x4<f32>,
                camera_position: vec3<f32>,
                _padding1: f32,
                camera_direction: vec3<f32>,
                _padding2: f32,
                log_depth_constant: f32,
                far_plane_distance: f32,
                near_plane_distance: f32,
                fc_constant: f32,
                mv_matrix: mat4x4<f32>,
            }
            
            @group(0) @binding(0)
            var<uniform> mvp: StandardMVPUniform;
            
            struct VertexInput {
                @location(0) position: vec3<f32>,
            }
            
            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
            }
            
            @vertex
            fn vs_main(input: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                // Just render at the center of the screen as a tiny quad
                out.clip_position = vec4<f32>(0.0, 0.0, 0.5, 1.0);
                return out;
            }
            
            @fragment
            fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                // Invisible
                return vec4<f32>(0.0, 0.0, 0.0, 0.0);
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Simple Occlusion Proxy Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Use ONLY the generated default shader's MVP bind group layout (group 0)
        let mvp_bind_group_layout =
            crate::generated_shaders::default::bind_groups::BindGroup0::get_bind_group_layout(
                device,
            );

        // Create pipeline layout with only group 0
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Simple Proxy Pipeline Layout"),
            bind_group_layouts: &[&mvp_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Simple Occlusion Proxy Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 3 * 4, // 3 f32s
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    }],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::empty(), // Don't write to color buffer
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Ok(Self { render_pipeline })
    }
}
