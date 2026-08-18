// 高さ場から地形メッシュの初期頂点・UV・インデックス配列を生成する。
export interface 地形幾何初期データ {
    readonly 頂点配列: Float32Array
    readonly 法線配列: Float32Array
    readonly UV配列: Float32Array
    readonly 添字配列: Uint32Array
}

export function 地形幾何データを生成する(
    解像度: number,
    一辺のメートル: number,
    高さデータ: Float32Array,
): 地形幾何初期データ {
    const 頂点数 = 解像度 * 解像度
    const 頂点配列 = new Float32Array(頂点数 * 3)
    const 法線配列 = new Float32Array(頂点数 * 3)
    const UV配列 = new Float32Array(頂点数 * 2)

    const 格子間隔 = 一辺のメートル / (解像度 - 1)
    const 半分 = 一辺のメートル / 2

    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const 頂点添字 = gz * 解像度 + gx
            const x = gx * 格子間隔 - 半分
            const z = gz * 格子間隔 - 半分
            const y = 高さデータ[頂点添字] ?? 0

            頂点配列[頂点添字 * 3 + 0] = x
            頂点配列[頂点添字 * 3 + 1] = y
            頂点配列[頂点添字 * 3 + 2] = z

            法線配列[頂点添字 * 3 + 0] = 0
            法線配列[頂点添字 * 3 + 1] = 1
            法線配列[頂点添字 * 3 + 2] = 0

            UV配列[頂点添字 * 2 + 0] = gx / (解像度 - 1)
            UV配列[頂点添字 * 2 + 1] = 1.0 - gz / (解像度 - 1)
        }
    }

    const 四角数 = (解像度 - 1) * (解像度 - 1)
    const 添字配列 = new Uint32Array(四角数 * 6)
    let 添字位置 = 0

    for (let gz = 0; gz < 解像度 - 1; gz++) {
        for (let gx = 0; gx < 解像度 - 1; gx++) {
            const 左上 = gz * 解像度 + gx
            const 右上 = gz * 解像度 + (gx + 1)
            const 左下 = (gz + 1) * 解像度 + gx
            const 右下 = (gz + 1) * 解像度 + (gx + 1)

            添字配列[添字位置++] = 左上
            添字配列[添字位置++] = 左下
            添字配列[添字位置++] = 右上

            添字配列[添字位置++] = 右上
            添字配列[添字位置++] = 左下
            添字配列[添字位置++] = 右下
        }
    }

    return { 頂点配列, 法線配列, UV配列, 添字配列 }
}

// 既存の頂点配列バッファへ高さ場データを書き戻す。
export function 地形頂点高さを更新する(
    頂点配列: Float32Array,
    解像度: number,
    高さデータ: Float32Array,
): void {
    const 頂点数 = 解像度 * 解像度
    for (let i = 0; i < 頂点数; i++) {
        頂点配列[i * 3 + 1] = 高さデータ[i] ?? 0
    }
}
