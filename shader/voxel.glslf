#version 150 core

in vec3 v_position;
in vec3 v_normal;
in vec4 v_color;

out vec4 fragment;

uniform float time;

const vec3 light_pos = vec3(-3.0, 3.0, -3.0);
const float light_brightness = 5.0;
const vec3 light_color = vec3(0.0, 0.0, 1.0);

void main() {
	vec3 sun = vec3(cos(time), sin(time), cos(time));
	vec3 sun_color = vec3(1.0, 1.0, 1.0);

	vec3 normal = normalize(v_normal);

	vec3 light_dir = normalize(light_pos - v_position);

	float lambertian = max(dot(light_dir,normal), 0.0) * light_brightness;
	float sun_lambertian = max(dot(sun,normal), 0.0);

	float total = (lambertian + sun_lambertian);

	if (total > 0.05) {
		fragment = total * v_color;
	} else {
		fragment = v_color * 0.05;
	}
}