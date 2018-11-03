#version 330

uniform mat4 mvpMatrix;

in vec3 position;
in vec3 velocity;
in float life;

out vec4 color;

void main()
{
    float lifeScale = clamp(life / 10.0, 0, 1);
    color = vec4(velocity, lifeScale);
    gl_Position = mvpMatrix * vec4(position, 1.0);
    gl_PointSize = 5.0 * lifeScale;
}
