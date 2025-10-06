use bevy::{
    color::palettes::css::RED,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        FullscreenShader,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_graph::{RenderGraphExt, RenderLabel, ViewNode, ViewNodeRunner},
        render_resource::{
            binding_types::{sampler, texture_2d},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, Operations, PipelineCache,
            RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, Sampler,
            SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType,
        },
        renderer::RenderDevice,
        view::ViewTarget,
        RenderApp, RenderStartup,
    },
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PostProcessPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, PostProcessIdentify));
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_scale(Vec3::splat(64.0)),
    ));
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn init_post_process_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    asset_load: Res<AssetServer>,
    render_device: Res<RenderDevice>,
    fullscreen_shader: Res<FullscreenShader>,
) {
    let layout = render_device.create_bind_group_layout(
        "后置处理绑定组",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::VERTEX_FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
            ),
        ),
    );

    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    let shader = asset_load.load("shaders/post_processing.wgsl");

    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("后置处理管线".into()),
        layout: vec![layout.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader,
            shader_defs: vec![],
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..Default::default()
        }),
        ..Default::default()
    });
    commands.insert_resource(PostProcessPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}
#[derive(Component, ExtractComponent, Clone)]
struct PostProcessIdentify;

/// 该后置处理只能针对一个`Camera2d`中实体进行处理
#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = (&'static ViewTarget, &'static PostProcessIdentify);

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, _): bevy::ecs::query::QueryItem<'w, '_, Self::ViewQuery>,
        world: &'w bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        for _ in 0..2 {
            let post_process = view_target.post_process_write();
            let bind_group = render_context.render_device().create_bind_group(
                "后置处理绑定组",
                &post_process_pipeline.layout,
                &BindGroupEntries::sequential((
                    post_process.source,
                    &post_process_pipeline.sampler,
                )),
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("后置处理管道"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: post_process.destination,
                    resolve_target: None,
                    ops: Operations::default(),
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_render_pipeline(pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;
struct PostProcessPlugin;
impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<PostProcessIdentify>::default());
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(RenderStartup, init_post_process_pipeline);

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(Core2d, PostProcessLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    PostProcessLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }
}
