#version 330 core
in vec3 FragPos;
in vec3 Normal;

uniform vec3 light_dir = vec3(1.0);
uniform vec3 light_diffuse = vec3(0.5);

out vec4 FragColor;

void main()
{
    vec3 color = vec3(1.0, 0.5, 0.2);

    vec3 ambient = 0.1 * color;
    
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-light_dir);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light_diffuse * diff * color;

    vec3 result = ambient + diffuse;
    FragColor = vec4(result, 1.0);
}
