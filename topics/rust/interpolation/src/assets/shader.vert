#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 pr_matrix; // Mandatory
uniform mat4 vw_matrix = mat4(1.0);
uniform mat4 ml_matrix = mat4(1.0);

out vec3 FragPos;
out vec3 Normal;

void main()
{
    FragPos = vec3(ml_matrix * vec4(aPos, 1.0f));
    Normal = mat3(transpose(inverse(ml_matrix))) * aNormal;
    gl_Position = pr_matrix * vw_matrix * vec4(FragPos, 1.0f);
}
