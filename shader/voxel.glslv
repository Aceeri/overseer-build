#version 150 core

in vec4 voxel_color;
in ivec4 voxel_position;

in ivec4 vertex_position;
in ivec4 vertex_normal;

out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;

uniform mat4 camera_transform;

void main() {
	gl_Position = camera_transform * (vertex_position + 2 * voxel_position);

	v_color = voxel_color;
	v_normal = vec3(vertex_normal);
	v_position = vec3(vertex_position + 2 * voxel_position);
}