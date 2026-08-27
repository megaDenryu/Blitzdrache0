import { 楽曲編集状態, 初期楽曲を生成する } from './編集モデル/index.ts'
import { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { 楽曲編集ポインタ振り分け } from './楽曲編集ポインタ振り分け.ts'
import type { I楽曲発音配線 } from './画面/発音配線.ts'
import type { 升目の当たりの記録 } from './画面/打ち込み見込み.ts'

export interface テスト用入力環境 {
    readonly 状態: 楽曲編集状態
    readonly UI状態: 楽曲編集UI状態
    readonly 操作: 楽曲履歴適用サービス
    readonly 振り分け: 楽曲編集ポインタ振り分け
    readonly 発音記録: Array<{ トラックの位置: number, 行の位置: number }>
    readonly 見込み更新回数: { 値: number }
}

export function 楽曲入力一式を組む(初期進行外モード: boolean = false): テスト用入力環境 {
    const 状態 = new 楽曲編集状態(初期楽曲を生成する('テスト曲', 'テスト曲'))
    const UI状態 = new 楽曲編集UI状態()
    UI状態.進行の外モードか = 初期進行外モード

    const 見込み更新回数 = { 値: 0 }
    const 発音記録: Array<{ トラックの位置: number, 行の位置: number }> = []

    const 発音配線: I楽曲発音配線 = {
        升目の音を鳴らす: (トラックの位置, 行の位置) => {
            発音記録.push({ トラックの位置, 行の位置 })
        },
    }

    const 操作 = new 楽曲履歴適用サービス(状態)
    const 振り分け = new 楽曲編集ポインタ振り分け(
        状態,
        UI状態,
        操作,
        () => { 見込み更新回数.値++ },
        発音配線,
    )

    return { 状態, UI状態, 操作, 振り分け, 発音記録, 見込み更新回数 }
}

export function 升目当たりを作る(
    パターンの名乗り: string,
    トラックの位置: number,
    行の位置: number,
    ステップ: number,
): 升目の当たりの記録 {
    return { パターンの名乗り, トラックの位置, 行の位置, ステップ }
}
