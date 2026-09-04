import type { 粗マスの塗り } from '../../../../../生成/編集資源契約.ts'
import type { 地表材質色 } from '../../三次元/地形/地表材質色.ts'
import type { 見下ろし図の視点 } from '../見下ろし図の視点.ts'
import { 重ね描きの寸法, 重ね描きの配色, 高さの文字が収まるか, 高さの文字を描く } from '../重ね描きの配色.ts'
import { 高さを図の文字にする } from '../重ね描きの数値書式.ts'
import type { 粗マスの格子 } from './粗マスの座標変換.ts'

// 塗り一覧は下書きの一辺で、塗っている途中の番地は右サイドバーで選んでいる一辺で数えているため、格子を2つ受け取る。
export interface 粗マスの重ね描きの入力 {
    readonly 格子: 粗マスの格子
    readonly 塗っている途中の格子: 粗マスの格子
    readonly 塗り一覧: readonly 粗マスの塗り[]
    readonly 材質色: 地表材質色
    readonly 塗っている途中の番地一覧: ReadonlyArray<{ readonly 列: number; readonly 行: number }>
    readonly 格子線を描くか: boolean
    readonly 選択中の番地: { readonly 列: number; readonly 行: number } | null
}

// 粗マスの格子線と、塗られている粗マスを層の識別色(マテリアル台帳由来)の半透明と高さの数字で描く。
// 左ボタンを押してから離すまでに通った粗マスは、まだコマンドになっていないため縁だけで示す。
// 格子線は粗マスモードのときだけ描く。等高線を描くときに格子線が重なると線の位置が読みにくいためである。
export function 粗マスを重ね描きする(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 入力: 粗マスの重ね描きの入力): void {
    if (入力.格子線を描くか) 格子線を描く(文脈, 視点, 入力.塗っている途中の格子)
    for (const 塗り of 入力.塗り一覧) {
        const { 左上, 一辺 } = 粗マスの画素の矩形(視点, 入力.格子, 塗り)
        if (塗り.層 !== null) {
            文脈.globalAlpha = 重ね描きの寸法.粗マスの塗りの不透明度
            文脈.fillStyle = 入力.材質色[塗り.層]
            文脈.fillRect(左上.x, 左上.y, 一辺, 一辺)
            文脈.globalAlpha = 1
        }
        文脈.strokeStyle = 重ね描きの配色.粗マスの塗りの縁
        文脈.lineWidth = 重ね描きの寸法.粗マスの格子線の太さ画素
        文脈.strokeRect(左上.x, 左上.y, 一辺, 一辺)
        if (塗り.高さメートル !== null) {
            const 文 = 高さを図の文字にする(塗り.高さメートル)
            if (高さの文字が収まるか(文脈, 文, 一辺 * 重ね描きの寸法.高さの文字を描く升の幅の割合)) {
                高さの文字を描く(文脈, 文, 左上.x + 一辺 / 2, 左上.y + 一辺 / 2)
            }
        }
    }
    for (const 番地 of 入力.塗っている途中の番地一覧) {
        const { 左上, 一辺 } = 粗マスの画素の矩形(視点, 入力.塗っている途中の格子, 番地)
        文脈.strokeStyle = 重ね描きの配色.描いている途中の等高線
        文脈.lineWidth = 重ね描きの寸法.等高線の太さ画素
        文脈.strokeRect(左上.x, 左上.y, 一辺, 一辺)
    }
    if (入力.選択中の番地 !== null) {
        const { 左上, 一辺 } = 粗マスの画素の矩形(視点, 入力.格子, 入力.選択中の番地)
        文脈.strokeStyle = 重ね描きの配色.選択中の粗マスの枠
        文脈.lineWidth = 重ね描きの寸法.選択中の粗マスの枠の太さ画素
        文脈.strokeRect(左上.x, 左上.y, 一辺, 一辺)
    }
}

function 粗マスの画素の矩形(視点: 見下ろし図の視点, 格子: 粗マスの格子, 番地: { readonly 列: number; readonly 行: number }): { 左上: { x: number; y: number }; 一辺: number } {
    const 左上 = 視点.ワールドから画素へ(格子.粗マスの北西の角(番地))
    return { 左上, 一辺: 格子.粗マスの一辺のメートル() * 視点.画素毎メートル }
}

function 格子線を描く(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 格子: 粗マスの格子): void {
    const 数 = 格子.一辺に並ぶ粗マスの数()
    const 半分 = 格子.一辺のメートル / 2
    const 一辺 = 格子.粗マスの一辺のメートル()
    文脈.strokeStyle = 重ね描きの配色.粗マスの格子線
    文脈.lineWidth = 重ね描きの寸法.粗マスの格子線の太さ画素
    文脈.beginPath()
    for (let i = 1; i < 数; i++) {
        const 位置 = -半分 + i * 一辺
        const 縦の上 = 視点.ワールドから画素へ({ x: 位置, z: -半分 })
        const 縦の下 = 視点.ワールドから画素へ({ x: 位置, z: 半分 })
        const 横の左 = 視点.ワールドから画素へ({ x: -半分, z: 位置 })
        const 横の右 = 視点.ワールドから画素へ({ x: 半分, z: 位置 })
        文脈.moveTo(縦の上.x, 縦の上.y)
        文脈.lineTo(縦の下.x, 縦の下.y)
        文脈.moveTo(横の左.x, 横の左.y)
        文脈.lineTo(横の右.x, 横の右.y)
    }
    文脈.stroke()
}
