import { div, span, DivC } from 'sengen-ui'
import { 行コンテナ, 項目ラベル } from '../共通/スタイル.css.ts'
import type { パターン小節数欄 } from './パターン小節数欄.ts'
import type { パターン表示名入力欄 } from './パターン表示名入力欄.ts'
import type { パターンの進行選択欄 } from './パターンの進行選択欄.ts'
import { 編集グリッド } from './スタイル.css.ts'

export type パターン編集欄一式 = {
    readonly 表示名入力: パターン表示名入力欄
    readonly 小節数入力: パターン小節数欄
    readonly 進行選択: パターンの進行選択欄
}

// パターンの表示名・小節数・進行の3つの編集欄を、ラベル付きの行として並べたグリッドへ組む
// (パターンパネルからの工程分離)。呼び出し元は組み上がったDOM要素だけを受け取る。
export function パターン編集グリッドを組み立てる(欄一式: パターン編集欄一式): DivC {
    return div({ class: 編集グリッド }).childs([
        div({ class: 行コンテナ }).childs([span({ class: 項目ラベル, text: 'パターンの表示名' }), 欄一式.表示名入力]),
        div({ class: 行コンテナ }).childs([span({ class: 項目ラベル, text: '小節数' }), 欄一式.小節数入力]),
        div({ class: 行コンテナ }).childs([span({ class: 項目ラベル, text: 'コード進行' }), 欄一式.進行選択]),
    ])
}
