#version 330 core

in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

void main()
{
	vec3 norm = normalize(Normal);

	float ambientStrength = 0.1;
	vec3 ambient = ambientStrength * lightColor;

	vec3 lightDir = normalize(lightPos - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	float specularStrength = 0.5;
	int gloss = 32;	// powers of 2 preferable, less is more.
	vec3 viewDir = normalize(viewPos - FragPos);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), gloss);
	vec3 specular = specularStrength * spec * lightColor;

	vec3 colour = (ambient + diffuse + specular) * objectColor;
	FragColor = vec4(colour, 1.0);
}
