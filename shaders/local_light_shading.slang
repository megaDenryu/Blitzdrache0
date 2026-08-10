// 局所光の標本の列挙。受け取るのは画面の画素とビュー空間の奥行きと表面のカメラ相対位置、返すのはその画素へ
// 届く光の件数と、n件目の光の向きと放射である。材質の反射の式を1つも持たないため、PBRのシーンも両面ランバートの
// 布も同じこの1つを通して同じ並びの同じ光を読む(参照: `_doc/設計/放射輝度問い合わせ階層.md`「A. 直接光標本列挙」)。
//
// 消費側がクラスタ格子の作られ方を知らないことを2重に閉じている。
// 1つは、束縛先とレコードの形を持つモジュールをこのモジュールが取り込むだけで再輸出しないことであり、消費側からは資源の名前も型の名前も見えない。
// もう1つは、問い合わせの値の私有フィールドであり、区間の開始位置と件数は問い合わせの外から読めない。
// 消費側が呼べるのは、問い合わせを作る`ofScreenPixel`と、件数の`count`と、標本の`sampleAt`の3つだけである(参照: `_doc/設計/クラスタ多光源と点光源の影.md`の先行裁定「材質シェーダーは光源の並びとクラスタの生成方法を知らない」)。
//
// 放射に減衰の窓を掛けるのは、クラスタの打ち切りと絵を一致させるためである(窓そのものは
// `local_light_attenuation.slang`が持つ。正本は`crates/blitz_render/src/clustered_lighting/`の2つである)。
//
// 影の減衰もこの内側で掛ける。材質のシェーダーが受け取るのは影を掛けた後の放射であり、影の有無も方式も
// 名指しできない(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断o」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 取り込みを再輸出しないのは、これがこの分割の要だからである。消費側はこのモジュールを取り込んでも`local_light_binding`の4つの束縛先も`local_light_records`のレコードの形も見えず、下の問い合わせの型しか名指しできない。
// `lighting_query`はヘッダを読むために取り込む(消費側も自分で取り込んでよい値である)。
import cluster_mapping;
import lighting_query;
import local_light_attenuation;
import local_light_binding;
import point_light_shadow_sample;

// 距離の二乗へ足す微小量。CPU正本の`距離の二乗へ足す微小量`と同じ値でなければならない。
static const float localLightSquaredDistanceEpsilon = 1e-8;

// 1件の局所光がその表面へ届けるもの。向きは表面から光へ向かう単位ベクトルである。
public struct LocalLightSample
{
    public float3 directionToLight;
    public float3 radiance;
};

// その画素へ届く局所光の問い合わせ。
public struct LocalLightQuery
{
    // 注意: 2つとも私有である。クラスタの区間の表し方を消費側から閉じることがこの型の担当であり、
    // 消費側が触れるのは件数と標本だけである。
    private uint indexStart;
    private uint reachingLightCount;

    // その画素が読むセルを1度だけ求める。画面の外の画素も範囲の外の奥行きも端のセルへ入るため、どの画素も
    // 1つの問い合わせを得る。標本ごとにセルを求め直さないのは、区間がセルの添字だけで決まるためである。
    public static LocalLightQuery ofScreenPixel(uint2 pixel, float viewDepth)
    {
        uint3 divisions = lightingQueryHeader.clusterGridDivisions.xyz;
        uint2 extent = lightingQueryHeader.clusterScreenPixelExtent.xy;
        float2 depthRange = lightingQueryHeader.clusterDepthRangeAndHalfAngleTangent.xy;
        uint3 cellCoordinate = uint3(
            clusterTileIndexOfPixel(pixel.x, extent.x, divisions.x),
            clusterTileIndexOfPixel(pixel.y, extent.y, divisions.y),
            clusterDepthSliceOfViewDepth(viewDepth, depthRange.x, depthRange.y, divisions.z));
        ClusterCell cell = clusterCells[clusterFlatCellIndex(cellCoordinate, divisions)];
        LocalLightQuery query;
        query.indexStart = cell.start;
        query.reachingLightCount = cell.count;
        return query;
    }

    // この画素へ届く局所光の件数。
    public uint count()
    {
        return reachingLightCount;
    }

    // n件目の光がその表面へ届ける向きと放射。点光でない光種は偽を返して読み飛ばさせる。
    // 光種の枝を明示するのは、種別を読まずに点光の式を当てると、光種が増えたときに新しい種別の光を黙って
    // 点光の減衰で照らしてしまうためである。
    public bool sampleAt(uint slot, float3 relativePosition, out LocalLightSample result)
    {
        result.directionToLight = float3(0.0, 0.0, 0.0);
        result.radiance = float3(0.0, 0.0, 0.0);
        LocalLightRecord record = localLightRecords[clusterLightIndices[indexStart + slot]];
        if (record.kind.x != localLightKindPoint)
        {
            return false;
        }
        float3 toLight = record.cameraRelativePosition.xyz - relativePosition;
        float squaredDistance = dot(toLight, toLight);
        float lightDistance = sqrt(squaredDistance);
        // 光の位置にちょうど載った表面では光へ向かう向きが定まらず、影の立方体のどの面を読むかも決まらないため、零ベクトルを渡した比較標本が返す未定義の値(非数なら内積0を掛けても画素が汚れる)を避けて、標本を1つも行わずに向きも放射も0のまま返す。
        if (lightDistance <= 0.0)
        {
            return true;
        }
        float window = localLightAttenuationWindow(lightDistance, record.influenceRadius);
        result.directionToLight = toLight / lightDistance;
        result.radiance = record.colorAndIntensity.rgb * record.colorAndIntensity.w / (squaredDistance + localLightSquaredDistanceEpsilon) * window;
        // 影を落とす灯だけが立方体を1つ占める。影なしの灯は立方体を持たないため、添字を読まずに素通りする
        // (番兵の添字で影なしを表さない形の実効がこの枝である)。
        if (record.hasShadow != 0)
        {
            result.radiance *= pointLightShadowFactor(
                -toLight, record.influenceRadius, record.shadowResourceIndex,
                pointLightShadowTexture, pointLightShadowSampler);
        }
        return true;
    }
};
