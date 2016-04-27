#version 150 core

in vec4 voxel_Color;
in ivec4 voxel_Transform;
in ivec4 a_Pos;
out vec4 v_Color;

uniform mat4 u_Transform;

void main() {
  gl_Position = u_Transform * (a_Pos + 2 * voxel_Transform);

  v_Color = voxel_Color;
}