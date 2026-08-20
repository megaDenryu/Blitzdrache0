import { DoubleSide, MeshBasicMaterial } from 'three'

// 道路点マーカーの4つの状態それぞれの色。テーマごとに差し替えるため、部品の中へ直に書かない。
export interface 道路点マーカー配色 {
    readonly 通常色: number
    readonly アクティブ道色: number
    readonly 選択色: number
    readonly ドラッグ色: number
}

// 道路点マーカーが取りうる見た目の状態。アクティブな道の点とは、いま描き足す先になっている
// 道路に属する点のことであり、他の道の点と描き分ける。
export type 道路点マーカーの状態 = '通常' | 'アクティブな道の点' | '選択中' | 'つかんでいる'

// 4つの状態ぶんの材質をまとめて持ち、状態から材質を引く役と、テーマ切替で色を差し替える役を担う。
// 地形の起伏へ埋もれても見えるよう、いずれも奥行き判定を切ってある。
export class 道路点マーカー材質束 {
    private readonly _通常材質: MeshBasicMaterial
    private readonly _アクティブ道材質: MeshBasicMaterial
    private readonly _選択材質: MeshBasicMaterial
    private readonly _つかんでいる材質: MeshBasicMaterial

    public constructor(初期配色: 道路点マーカー配色) {
        this._通常材質 = 道路点マーカー材質束._材質を作る(初期配色.通常色)
        this._アクティブ道材質 = 道路点マーカー材質束._材質を作る(初期配色.アクティブ道色)
        this._選択材質 = 道路点マーカー材質束._材質を作る(初期配色.選択色)
        this._つかんでいる材質 = 道路点マーカー材質束._材質を作る(初期配色.ドラッグ色)
    }

    public get 材質一覧(): readonly MeshBasicMaterial[] {
        return [this._通常材質, this._アクティブ道材質, this._選択材質, this._つかんでいる材質]
    }

    public 状態の材質を取り出す(状態: 道路点マーカーの状態): MeshBasicMaterial {
        if (状態 === '選択中') return this._選択材質
        if (状態 === 'つかんでいる') return this._つかんでいる材質
        if (状態 === 'アクティブな道の点') return this._アクティブ道材質
        return this._通常材質
    }

    public 配色を設定する(配色: 道路点マーカー配色): void {
        this._通常材質.color.set(配色.通常色)
        this._アクティブ道材質.color.set(配色.アクティブ道色)
        this._選択材質.color.set(配色.選択色)
        this._つかんでいる材質.color.set(配色.ドラッグ色)
    }

    private static _材質を作る(色: number): MeshBasicMaterial {
        return new MeshBasicMaterial({
            color: 色,
            side: DoubleSide,
            transparent: true,
            opacity: 0.9,
            depthTest: false,
        })
    }
}
