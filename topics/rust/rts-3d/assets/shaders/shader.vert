
#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 pr_matrix; // Mandatory
uniform mat4 vw_matrix = mat4(1.0);
uniform mat4 ml_matrix = mat4(1.0);

out vec3 FragPos;

void main()
{
    gl_Position = pr_matrix * vw_matrix * ml_matrix * vec4(aPos, 1.0f);
}
