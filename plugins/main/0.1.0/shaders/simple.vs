#import "version_directive.glsl"
#import "vertex_layout.glsl"

void main()
{
  io_uv       = i_uv * u_tex_ratio + u_tex_coords;
  gl_Position = u_projection * u_view * u_model * vec4(i_pos, 1.0);
}
