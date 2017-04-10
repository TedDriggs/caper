
/// OpenGL 3.3 default post shaders
pub mod gl330 {
    /// Default post vertex shader
    pub const VERT: &'static str =
        "
        #version 330

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec2 texture;

        out vec2 v_tex_coords;

        void main() {
            gl_Position = vec4(position, 1.0);
            v_tex_coords = texture;
        }
    ";
    /// Default post fragment shader
    pub const FRAG: &'static str =
        "
        #version 330

        #define M_PI 3.1415926535897932384626433832795

        uniform vec2 resolution;
        uniform sampler2D tex;
        uniform sampler2D depth_buf;

        // chromatic aberration params
        uniform float chrom_amt;
        uniform float chrom_offset;

        in vec2 v_tex_coords;

        out vec4 frag_output;

        float offset[3] = float[]( 0.0, 1.5, 3.0 );
        float weight[3] = float[]( 0.1, 0.15, 0.03 );

        void main() {
            vec4 color = texture(tex, v_tex_coords);
            float depth = texture(depth_buf, v_tex_coords).r;

            // pseudo chromatic aberration
            if (chrom_amt > 0.0) {
                vec2 edge_offset = vec2(cos(v_tex_coords.x * M_PI), cos(v_tex_coords.y * M_PI)) * chrom_offset;
                float chrom_r = texture(tex, v_tex_coords + edge_offset).r;
                color.r = mix(color.r, chrom_r, chrom_amt);
            }

            // blur
            vec3 tc = color.rgb;
            for (int i=1; i<3; i++) {
                tc += texture(tex, v_tex_coords + vec2(0.0, offset[i]) / resolution.y).rgb * weight[i];
                tc += texture(tex, v_tex_coords - vec2(0.0, offset[i]) / resolution.y).rgb * weight[i];
                tc += texture(tex, v_tex_coords + vec2(offset[i], 0.0) / resolution.x).rgb * weight[i];
                tc += texture(tex, v_tex_coords - vec2(offset[i], 0.0) / resolution.x).rgb * weight[i];
                tc += texture(tex, v_tex_coords + vec2(offset[i] / resolution.x, offset[i] / resolution.y)).rgb * weight[i];
                tc += texture(tex, v_tex_coords - vec2(offset[i] / resolution.x, offset[i] / resolution.y)).rgb * weight[i];
            }

            // mix with depth buffer for bokeh
            frag_output = mix(color, vec4(tc, 1.0), sin(depth * M_PI / 2.0));
        }
    ";
}
