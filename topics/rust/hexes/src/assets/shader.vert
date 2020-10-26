#version 450

layout(set = 0, binding = 0) uniform Uniforms {
    mat4x4 projection_view;
    vec4 clipping_plane;
};

layout(location = 0) in vec3 position;
// For lighting later
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 colour;

layout(location = 0) out vec4 v_Colour;
layout(location = 1) out float v_ClipDist;

void main() {
    gl_Position = projection_view * vec4(position, 1.0);

    v_Colour = colour;
    v_ClipDist = dot(vec4(position, 1.0), clipping_plane);
}
