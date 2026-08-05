// 深度プリパスが書いた画面1枚ぶんの深度を、局所可視性の2つのコンピュートが同じ形で読むための入口。
// 担うのは位置1つでの参照と、ジオメトリが描かれていない画素の判別だけである。
// 注意: 消去値は`crates/blitz_render/src/local_visibility/depth_image.rs`の`深度の消去値`と、
// `crates/blitz_render/src/vulkan/graph/rendering_setup.rs`の深度の消去値と同じでなければならない。
// 違えると空の画素が遮蔽の標本として読まれ、画面全体へ偽の遮蔽が乗る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 深度プリパスの消去値。この値のままの画素にはジオメトリが1つも描かれていない。
static const float localVisibilityDepthClearValue = 1.0;

// 画素1つの深度。
// 前提: 呼び出し元は画素が寸法の内側にあることを確かめている。
float localVisibilityDepthAt(Texture2D<float> depthImage, uint2 pixel)
{
    return depthImage.Load(int3(int(pixel.x), int(pixel.y), 0));
}

// その画素にジオメトリが描かれているか。消去値のままの画素は空であり、遮蔽の標本にも標点にもしない。
bool localVisibilityHasGeometry(Texture2D<float> depthImage, uint2 pixel)
{
    return localVisibilityDepthAt(depthImage, pixel) < localVisibilityDepthClearValue;
}

// 画素が寸法の内側にあるか。
bool localVisibilityContains(uint2 extent, int2 pixel)
{
    return pixel.x >= 0 && pixel.y >= 0 && uint(pixel.x) < extent.x && uint(pixel.y) < extent.y;
}
