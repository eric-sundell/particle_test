#version 330

uniform mat4 mvpMatrix;

in vec3 position;
in vec3 velocity;
in float life;

out vec3 vVelocity;

void main()
{
    vVelocity = velocity;
    gl_Position = mvpMatrix * vec4(position, 1.0);
}
