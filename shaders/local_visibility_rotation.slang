// 標本の方位を画素ごとにずらす回転と、その回転を選ぶ画素の位置の整数ハッシュ。
// 回転を画素ごとに変えるのは、全画素で同じ方位を使うと少ない標本数の偏りが同じ形の帯として画面へ焼き付くためである。
// 正弦を使う混ぜ方を採らないのは、超越関数の近似が機材ごとに違い、同じ画素が機材によって別の回転を選ぶためである。
// 整数の乗算とビットずらしだけなら、どの機材でも同じ答えが出る。
// 注意: この混ぜ方と回転は`crates/blitz_render/src/local_visibility/pixel_hash.rs`と`rotation.rs`の写しである。
// 正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 用意する回転の本数。両側ぼかしの既定の窓(5画素四方の25画素)がこの本数を覆える広さである。
static const uint localVisibilityRotationCount = 16;
// 一周のラジアン。
static const float localVisibilityFullTurn = 6.28318530717958647692;

// 横の位置に掛ける奇数。黄金比の逆数を32ビットへ写した値であり、隣り合う位置を最も離す。
static const uint localVisibilityHorizontalMultiplier = 0x9E3779B1;
// 縦の位置に掛ける奇数。横の乗数と互いに素な別の混ぜ合わせを与える。
static const uint localVisibilityVerticalMultiplier = 0x85EBCA77;
// 混ぜ合わせの段で掛ける奇数の組。上位のビットを下位へ折り返しながら乗算を挟む。
static const uint localVisibilityMixMultiplierFirst = 0x2C1B3C6D;
static const uint localVisibilityMixMultiplierSecond = 0x297A2D39;

// 画素の位置から決まる整数。同じ位置なら必ず同じ値を返す。
uint localVisibilityPixelHash(uint2 pixel)
{
    uint mixed = (pixel.x * localVisibilityHorizontalMultiplier) ^ (pixel.y * localVisibilityVerticalMultiplier);
    mixed ^= mixed >> 15;
    mixed *= localVisibilityMixMultiplierFirst;
    mixed ^= mixed >> 13;
    mixed *= localVisibilityMixMultiplierSecond;
    mixed ^= mixed >> 16;
    return mixed;
}

// 接空間の方位を回す1つの回転。
struct LocalVisibilityRotation
{
    float cosine;
    float sine;
};

// 番号が表す回転。全周を本数で等分した角を割り当てる。
// 番号から角を作る余弦と正弦は超越関数を通るが、入力が定数の角であるため機材ごとの近似の差は最下位のビットに留まる。
LocalVisibilityRotation localVisibilityRotationOfIndex(uint index)
{
    float angle = localVisibilityFullTurn * float(index) / float(localVisibilityRotationCount);
    LocalVisibilityRotation rotation;
    rotation.cosine = cos(angle);
    rotation.sine = sin(angle);
    return rotation;
}

// その画素が使う回転。同じ位置なら必ず同じ回転を返す。
LocalVisibilityRotation localVisibilityRotationOfPixel(uint2 pixel)
{
    return localVisibilityRotationOfIndex(localVisibilityPixelHash(pixel) % localVisibilityRotationCount);
}

// 接空間の平面の向きをこの回転の角だけ回した向き。
float2 localVisibilityRotatePlane(LocalVisibilityRotation rotation, float2 direction)
{
    return float2(
        direction.x * rotation.cosine - direction.y * rotation.sine,
        direction.x * rotation.sine + direction.y * rotation.cosine);
}
