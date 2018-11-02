#version 330

uniform mat4 mvpMatrix;

in vec3 position;
in vec3 velocity;

out vec3 color;

void main()
{
    color = velocity;
    gl_Position = mvpMatrix * vec4(position, 1.0);
}
