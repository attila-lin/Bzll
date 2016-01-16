#version 300 es
// matrix to convert a_position from model space to normalized device space
uniform mat4 u_mvpMatrix;

// attributes input to the vertex shader
in vec4 a_position;
in vec4 a_color;

out vec4 v_color;
void main()
{
  v_color = a_color;
  gl_Position = u_mvpMatrix * a_position;
}
