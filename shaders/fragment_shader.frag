#version 330 core
out vec4 FragColor;

in vec3 poscolor;

uniform float running_time;

void main()
{
    FragColor = vec4(poscolor + running_time, 1.0);
} 