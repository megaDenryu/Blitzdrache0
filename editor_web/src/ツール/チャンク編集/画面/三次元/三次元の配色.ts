import { テーマ登録簿, 既定テーマ名 } from '../../../../境界/index.ts'
import type { 道路点マーカー配色 } from './道路/道路点マーカー材質束.ts'

// 三次元ビューが持つ材質の色をまとめたもの。テーマ切替のたびに丸ごと差し替える。
// 大域編集ビューには植生が無いため、植生色は受け取っても使われない。
export interface 三次元の配色 {
    readonly 地形基本色: number
    readonly 植生色: number
    readonly 道路色: number
    readonly 標高低色: number
    readonly 標高中色: number
    readonly 標高高色: number
    readonly 道路点マーカー配色: 道路点マーカー配色
}

// テーマ定義のうち、三次元ビューが読む色トークンだけを取り出した形。
export interface 三次元の色トークン {
    readonly 地形基本色: number
    readonly 植生色: number
    readonly 道路色: number
    readonly 標高低色: number
    readonly 標高中色: number
    readonly 標高高色: number
    readonly 道路点通常色: number
    readonly 道路点選択色: number
    readonly 道路点つかんでいる色: number
}

// 平らに並んだテーマの色トークンを、三次元ビューの部品が受け取る形へ組み替える。
export function 色トークンから三次元の配色を作る(トークン: 三次元の色トークン): 三次元の配色 {
    return {
        地形基本色: トークン.地形基本色,
        植生色: トークン.植生色,
        道路色: トークン.道路色,
        標高低色: トークン.標高低色,
        標高中色: トークン.標高中色,
        標高高色: トークン.標高高色,
        道路点マーカー配色: {
            通常色: トークン.道路点通常色,
            選択色: トークン.道路点選択色,
            ドラッグ色: トークン.道路点つかんでいる色,
        },
    }
}

// 部品の構築時、まだテーマを受け取っていない一瞬のあいだ使う配色。既定テーマの値を引くため、
// 三次元ビューの中に色の数値を書かずに済む。構築直後にタブ開閉サービスが本来のテーマを適用する。
export function 既定の三次元の配色を作る(): 三次元の配色 {
    const 既定テーマ = テーマ登録簿.get(既定テーマ名)
    if (既定テーマ === undefined) {
        throw new Error(`既定テーマ「${既定テーマ名}」がテーマ登録簿に登録されていない`)
    }
    return 色トークンから三次元の配色を作る(既定テーマ)
}
