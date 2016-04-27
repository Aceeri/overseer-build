#version 150 core

in vec4 voxel_color;
in ivec4 voxel_position;
in ivec4 vertex_position;
in ivec4 vertex_normal;

out vec4 vertex_color;

uniform mat4 camera_transform;

void main() {
  gl_Position = camera_transform * (vertex_position + 2 * voxel_position);

  vertex_color = voxel_color;
}