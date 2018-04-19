#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_explicit_attrib_location : require

layout(location = 0) in vec2 v_Uv;
layout(location = 0, index = 0) out vec4 Target0;
layout(location = 0, index = 1) out vec4 Target1;

layout(set = 0, binding = 0) uniform texture2D u_Texture;
layout(set = 0, binding = 1) uniform sampler u_Sampler;

void main() {
    Target0 = texture(sampler2D(u_Texture, u_Sampler), v_Uv);
    Target1 = texture(sampler2D(u_Texture, u_Sampler), v_Uv);
}
