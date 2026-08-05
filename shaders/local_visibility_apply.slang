// 局所可視性補正を消費する側の唯一の境界。両方の照明問い合わせ契約の画素段(定数近似・遠方環境)が
// 取り込む先はこの1つのモジュールであり、同じ画像を同じ座標で読み、同じ規約で拡散間接照度へ掛ける。
// 契約ごとに別の掛け方を書けないようにすることがこのモジュールの担当である。
//
// 掛ける先を拡散間接だけに限るのは、局所可視性が「近傍の形が半球のどれだけを塞いでいるか」であり、
// その量が意味を持つのは半球全体から一様に集める拡散の項だけだからである。直接光は影が担い、鏡面間接は
// 反射方向の細い立体角を見るため、同じ係数を掛ける根拠が無い。最終色へ後掛けする道をこのモジュールが
// 1つも持たないのは、後掛けが直接光と発光まで暗くしてしまうためである。
// 参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」。
//
// 拡散間接方式が環境のみの世界では、局所可視度1だけを持つ画像が束縛される。8ビット無符号正規化の符号値255は
// ちょうど1.0であるため、掛け算が値を1ビットも変えない。方式で枝を分けずに常に掛けるのは、
// 定数近似とSSAOの組み合わせを不正状態にしないためである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 照明問い合わせのセット(set3)のbinding7。両方の契約のシーン画素段が同じ番号で宣言する。
[[vk::binding(7, 3)]] Texture2D<float> localVisibilityImage;

// その画素の局所可視度。1が遮蔽なしである。
// 前提: 画面座標は画素段の組み込み出力`SV_Position`であり、画像は画面と同じ寸法で焼かれている。
float localVisibilityOfScreenPosition(float4 screenPosition)
{
    return localVisibilityImage.Load(int3(int(screenPosition.x), int(screenPosition.y), 0)).r;
}

// 拡散間接照度へ局所可視度を掛けた値。直接光・鏡面間接・発光はこの関数を1度も通らない。
float3 applyLocalVisibility(float3 diffuseIndirect, float localVisibility)
{
    return diffuseIndirect * localVisibility;
}
