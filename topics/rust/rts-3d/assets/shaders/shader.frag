#version 330 core
in vec4 position;

uniform vec2 light_pos = vec2(1.0);

out vec4 FragColor;

void main()
{
    vec4 color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    float intensity = 1.0 / length(position.xy - light_pos);
    FragColor = color * intensity;
} 
