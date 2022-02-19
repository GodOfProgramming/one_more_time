#import "version_directive.glsl"
#import "vertex_layout.glsl"

out vec2 tex_coords;
out vec3 color;

void main()
{
  tex_coords = uv;
  color = norm;
  gl_Position = vec4(pos.xyz, 1.0);
}
