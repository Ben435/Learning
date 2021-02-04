
#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    // Flip, as texture coords up = positive.
    vec2 invertedTexCoords = vec2(v_tex_coords.x, -v_tex_coords.y);
    f_color = texture(
        sampler2D(t_diffuse, s_diffuse), 
        invertedTexCoords
    );
}
