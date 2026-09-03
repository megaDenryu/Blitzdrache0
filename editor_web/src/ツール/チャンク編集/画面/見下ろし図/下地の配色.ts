import type { 地表材質色 } from '../三次元/地形/地表材質色.ts'

// 見下ろし図の下地が使う色。地表4層の識別色はマテリアル台帳が正本(判断9)で三次元と同じ値を受け取り、
// 標高の3段はテーマの標高グラデーションを受け取る。どちらも三次元の地形と同じ出どころにするため、
// この部品の中に色の数値を書かない。
export interface 下地の配色 {
    readonly 材質色: 地表材質色
    readonly 標高低色: number
    readonly 標高中色: number
    readonly 標高高色: number
}

export interface RGB {
    readonly r: number
    readonly g: number
    readonly b: number
}

// 0xRRGGBB の整数を3成分へ分ける。
export function 整数の色をRGBへ(色: number): RGB {
    return { r: (色 >> 16) & 0xff, g: (色 >> 8) & 0xff, b: 色 & 0xff }
}

// "#rrggbb" の文字列を3成分へ分ける。書式が違うときは灰色を返さず例外にする(無言のデフォルト適用の禁止)。
export function 十六進の色文字列をRGBへ(色: string): RGB {
    const 一致 = /^#([0-9a-fA-F]{6})$/.exec(色)
    if (一致 === null || 一致[1] === undefined) {
        throw new Error(`識別色は "#rrggbb" の形でなければならない: ${色}`)
    }
    return 整数の色をRGBへ(Number.parseInt(一致[1], 16))
}

export function RGBを混ぜる(甲: RGB, 乙: RGB, 乙の割合: number): RGB {
    const t = Math.min(1, Math.max(0, 乙の割合))
    return {
        r: 甲.r + (乙.r - 甲.r) * t,
        g: 甲.g + (乙.g - 甲.g) * t,
        b: 甲.b + (乙.b - 甲.b) * t,
    }
}

// 三次元の地形シェーダーの elevationGradientColor と同じ規則で、正規化した高さ(0..1)を3段の色へ写す。
// 参照: `画面/三次元/地形/地形シェーダーのGLSL.ts`
export function 標高グラデーションの色(配色: 下地の配色, 正規化した高さ: number): RGB {
    const t = Math.min(1, Math.max(0, 正規化した高さ))
    if (t < 0.5) {
        return RGBを混ぜる(整数の色をRGBへ(配色.標高低色), 整数の色をRGBへ(配色.標高中色), t * 2)
    }
    return RGBを混ぜる(整数の色をRGBへ(配色.標高中色), 整数の色をRGBへ(配色.標高高色), (t - 0.5) * 2)
}
