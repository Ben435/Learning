#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

/*
Hexagon
*/
vec2 positions[6] = vec2[](
    vec2(-1, 0),
    vec2(-0.5, 0.9),
    vec2(0.5, 0.9),
    vec2(1, 0),
    vec2(0.5, -0.9),
    vec2(-0.5, -0.9)
);

int indices[4][3] = int[][](
    int[](1, 5, 0),
    int[](3, 4, 5),
    int[](1, 2, 3),
    int[](1, 3, 5)
);

void main() {
    int x_index = gl_VertexIndex / 3;
    int y_index = gl_VertexIndex % 3;
    int index = indices[x_index][y_index];
    vec2 position = positions[index];
    gl_Position = vec4(position, 0.0, 1.0);
}
