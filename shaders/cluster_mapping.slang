// クラスタ格子の写像の算術。束縛先を1つも読まず、受け取るのは写像のパラメータと画素または添字だけである。
// 画素段(セルを引く向き)と選別のコンピュート(セルの錐台を作る向き)の両方がこの1つを取り込む。
// 注意: この式は`crates/blitz_render/src/clustered_lighting/`の写しである。正本はRust側にあり、
// 単精度で組んだ同じ式との食い違いを`single_precision_agreement_tests`が測っている。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 実数の位置以下で最大の添字。非数と負は0へ、上限より大きい位置は上限へ張り付ける。
// 正本は`crates/blitz_render/src/numeric/floor_index.rs`である。
uint clusterFloorIndex(float realPosition, uint maxIndex)
{
    if (isnan(realPosition) || realPosition <= 0.0)
    {
        return 0;
    }
    if (realPosition >= float(maxIndex))
    {
        return maxIndex;
    }
    return min(uint(floor(realPosition)), maxIndex);
}

// 画素が属するタイルの添字。画素は中心で所属を決め、画素数の外へ出た画素は端のタイルへ入れる。
// 注意: 積を32ビットで作る。画素数は交換列が作れる寸法(2の14乗以下)、分割数は仮置きで16以下であり、
// 積は2の20乗を超えない。
uint clusterTileIndexOfPixel(uint pixel, uint extent, uint divisions)
{
    return min((2 * pixel + 1) * divisions / (2 * extent), divisions - 1);
}

// 奥行きが属する段の添字。近より手前は段0、遠より奥は最後の段へ入れる。
uint clusterDepthSliceOfViewDepth(float viewDepth, float nearDepth, float farDepth, uint divisions)
{
    float realSlice = log(viewDepth / nearDepth) / log(farDepth / nearDepth) * float(divisions);
    return clusterFloorIndex(realSlice, divisions - 1);
}

// 段の境界の奥行き。境界の番号は0から分割数までを取る。
float clusterSliceBoundViewDepth(uint boundNumber, float nearDepth, float farDepth, uint divisions)
{
    return nearDepth * pow(farDepth / nearDepth, float(boundNumber) / float(divisions));
}

// タイルが占める正規化装置座標の区間。xが左端、yが右端であり、左端が-1、右端が+1である。
float2 clusterTileNdcRange(uint index, uint divisions)
{
    float tileWidth = 2.0 / float(divisions);
    float lower = float(index) * tileWidth - 1.0;
    return float2(lower, lower + tileWidth);
}

// 3軸の添字を1本の添字へ畳む。横が最も速く変わり、次に縦、最後に奥行きである。
uint clusterFlatCellIndex(uint3 cell, uint3 divisions)
{
    return (cell.z * divisions.y + cell.y) * divisions.x + cell.x;
}

// 1本の添字を3軸の添字へ戻す。畳む向きと同じ規約を使う。
uint3 clusterCellOfFlatIndex(uint flatIndex, uint3 divisions)
{
    return uint3(
        flatIndex % divisions.x,
        (flatIndex / divisions.x) % divisions.y,
        flatIndex / (divisions.x * divisions.y));
}

// そのセルが使うクラスタ光添字列の開始位置。セルごとに同じ長さの区間を割り当てるため、開始位置は添字で決まる。
uint clusterLightIndexStart(uint flatIndex, uint perCellCapacity)
{
    return flatIndex * perCellCapacity;
}
