#version 450

layout(location = 0) in vec2 uv_Position;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform texture2D PixelChunkMaterial_texture;

layout(set = 2, binding = 1) uniform sampler PixelChunkMaterial_texture_sampler;

void main() {
    o_Target = texture(sampler2D(PixelChunkMaterial_texture, PixelChunkMaterial_texture_sampler), uv_Position);
}
