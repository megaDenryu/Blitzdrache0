import {
    type パターン,
    type 打楽器の種類,
    type 楽器,
    type 楽曲,
} from '../../../生成/編集資源契約.ts'
import { 数値からセルへ変換する } from './セル.ts'

// 演奏予定表へ1件ずつ渡すために取り出した、ある瞬間に鳴り始める音1つ分の指定。
export type 鳴り始める音 =
    | {
          readonly 種類: '音高の音'
          readonly トラックの位置: number
          readonly 楽器: 楽器
          readonly 音高番号: number
          readonly 長さのステップ数: number
      }
    | {
          readonly 種類: '打楽器の音'
          readonly トラックの位置: number
          readonly 楽器: 楽器
          readonly 打楽器: 打楽器の種類
          readonly 長さのステップ数: number
      }

// 打点から後ろへ続く継続のセルを数え、鳴らす長さをステップの数で求める。上限はこの行自身の長さである。
function 音の長さのステップ数を数える(行: readonly number[], 打点のステップ: number): number {
    let 長さ = 1
    for (let ステップ = 打点のステップ + 1; ステップ < 行.length; ステップ++) {
        const 生値 = 行[ステップ]
        if (生値 === undefined) break
        if (数値からセルへ変換する(生値).種類 !== '音の継続') break
        長さ++
    }
    return 長さ
}

function 行の音を組み立てる(
    楽曲: 楽曲,
    トラックの位置: number,
    行の位置: number,
    長さのステップ数: number,
): 鳴り始める音 {
    const トラック = 楽曲.トラック構成[トラックの位置]
    if (トラック === undefined) {
        throw new Error(`トラックがありません: トラックの位置=${トラックの位置}`)
    }
    if (トラック.音の並び.種類 === '打楽器の行一覧') {
        const 打楽器 = トラック.音の並び.値[行の位置]
        if (打楽器 === undefined) {
            throw new Error(`打楽器の行がありません: 行の位置=${行の位置}`)
        }
        return { 種類: '打楽器の音', トラックの位置, 楽器: トラック.楽器, 打楽器, 長さのステップ数 }
    }
    const 音高番号 = トラック.音の並び.値[行の位置]
    if (音高番号 === undefined) {
        throw new Error(`音高の行がありません: 行の位置=${行の位置}`)
    }
    return { 種類: '音高の音', トラックの位置, 楽器: トラック.楽器, 音高番号, 長さのステップ数 }
}

// 升目を押したときに1回だけ鳴らす音。長さはステップ1つ分とする。
export function 升目の音を組み立てる(
    楽曲: 楽曲,
    トラックの位置: number,
    行の位置: number,
): 鳴り始める音 {
    return 行の音を組み立てる(楽曲, トラックの位置, 行の位置, 1)
}

// パターンの指定したステップで鳴り始める音を、全トラック・全行から集める。
export function パターンのステップで鳴り始める音一覧を求める(
    楽曲: 楽曲,
    対象のパターン: パターン,
    パターン内ステップ: number,
): readonly 鳴り始める音[] {
    const 音一覧: 鳴り始める音[] = []
    for (let トラックの位置 = 0; トラックの位置 < 対象のパターン.格子.length; トラックの位置++) {
        const トラック格子 = 対象のパターン.格子[トラックの位置]
        if (トラック格子 === undefined) continue
        for (let 行の位置 = 0; 行の位置 < トラック格子.行一覧.length; 行の位置++) {
            const 行 = トラック格子.行一覧[行の位置]
            if (行 === undefined) continue
            const 生値 = 行[パターン内ステップ]
            if (生値 === undefined) continue
            if (数値からセルへ変換する(生値).種類 !== '音の始まり') continue
            const 長さのステップ数 = 音の長さのステップ数を数える(行, パターン内ステップ)
            音一覧.push(行の音を組み立てる(楽曲, トラックの位置, 行の位置, 長さのステップ数))
        }
    }
    return 音一覧
}
