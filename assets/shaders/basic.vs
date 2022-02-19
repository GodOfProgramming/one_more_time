#import "version_directive.glsl"
#import "vertex_layout.glsl"

out vec3 color;

void main()
{
  color = norm;
  gl_Position = vec4(pos.xyz, 1.0);
}
