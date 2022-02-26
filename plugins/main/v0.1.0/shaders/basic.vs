#import "main.version_directive.glsl"
#import "main.vertex_layout.glsl"

out vec2 tex_coords;
out vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
  tex_coords = uv;
  color = norm;
  gl_Position = projection * view * model * vec4(pos.xyz, 1.0);
}
