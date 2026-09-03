import type { 等高線, 平面の位置 } from '../../../../../生成/編集資源契約.ts'
import type { 見下ろし図の視点 } from '../見下ろし図の視点.ts'
import { 重ね描きの寸法, 重ね描きの配色, 高さの文字を描く } from '../重ね描きの配色.ts'
import { 高さを図の文字にする } from '../重ね描きの数値書式.ts'

// 重ね描きが毎回読むもの。等高線一覧は下書きから、選択と途中の線は画面の状態から読む。
export interface 等高線の重ね描きの入力 {
    readonly 等高線一覧: readonly 等高線[]
    readonly 選択中の添字: number | null
    readonly 描いている途中の頂点列: readonly 平面の位置[]
}

// 全等高線を線と頂点の丸で描き、線の近くに高さの数字を添える。選択中は色を変え、描いている途中の線は破線で描く。
export function 等高線を重ね描きする(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 入力: 等高線の重ね描きの入力): void {
    入力.等高線一覧.forEach((線, 添字) => {
        const 色 = 添字 === 入力.選択中の添字 ? 重ね描きの配色.選択中の等高線 : 重ね描きの配色.等高線
        文脈.setLineDash([])
        折れ線を描く(文脈, 視点, 線.頂点列, 線.閉じている, 色)
        頂点の丸を描く(文脈, 視点, 線.頂点列, 色)
        高さを添える(文脈, 視点, 線)
    })
    if (入力.描いている途中の頂点列.length > 0) {
        文脈.setLineDash([重ね描きの寸法.破線の刻み画素, 重ね描きの寸法.破線の刻み画素])
        折れ線を描く(文脈, 視点, 入力.描いている途中の頂点列, false, 重ね描きの配色.描いている途中の等高線)
        文脈.setLineDash([])
        頂点の丸を描く(文脈, 視点, 入力.描いている途中の頂点列, 重ね描きの配色.描いている途中の等高線)
    }
}

function 折れ線を描く(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 頂点列: readonly 平面の位置[], 閉じる: boolean, 色: string): void {
    if (頂点列.length < 2) return
    文脈.beginPath()
    頂点列.forEach((頂点, i) => {
        const 画素 = 視点.ワールドから画素へ(頂点)
        if (i === 0) 文脈.moveTo(画素.x, 画素.y)
        else 文脈.lineTo(画素.x, 画素.y)
    })
    if (閉じる) 文脈.closePath()
    文脈.strokeStyle = 色
    文脈.lineWidth = 重ね描きの寸法.等高線の太さ画素
    文脈.stroke()
}

function 頂点の丸を描く(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 頂点列: readonly 平面の位置[], 色: string): void {
    for (const 頂点 of 頂点列) {
        const 画素 = 視点.ワールドから画素へ(頂点)
        文脈.beginPath()
        文脈.arc(画素.x, 画素.y, 重ね描きの寸法.頂点の半径画素, 0, Math.PI * 2)
        文脈.fillStyle = 色
        文脈.fill()
        文脈.strokeStyle = 重ね描きの配色.等高線の頂点の縁
        文脈.lineWidth = 1
        文脈.stroke()
    }
}

// 高さの数字は先頭の頂点のすぐ上に置く。線の途中に置くと隣の線の数字と重なりやすいためである。
function 高さを添える(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点, 線: 等高線): void {
    const 先頭 = 線.頂点列[0]
    if (先頭 === undefined) return
    const 画素 = 視点.ワールドから画素へ(先頭)
    高さの文字を描く(文脈, `${高さを図の文字にする(線.高さメートル)}m`, 画素.x, 画素.y - 重ね描きの寸法.頂点の半径画素 - 重ね描きの寸法.文字の大きさ画素 / 2)
}
