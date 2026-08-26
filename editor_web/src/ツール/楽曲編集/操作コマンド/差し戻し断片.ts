import type { 打ち込み差し戻し断片 } from './差し戻し断片/打ち込み差し戻し.ts'
import type { 楽曲属性差し戻し断片 } from './差し戻し断片/楽曲属性差し戻し.ts'
import type { パターン差し戻し断片 } from './差し戻し断片/パターン差し戻し.ts'
import type { コード進行と曲構成差し戻し断片 } from './差し戻し断片/コード進行と曲構成差し戻し.ts'

export type * from './差し戻し断片/打ち込み差し戻し.ts'
export type * from './差し戻し断片/楽曲属性差し戻し.ts'
export type * from './差し戻し断片/パターン差し戻し.ts'
export type * from './差し戻し断片/コード進行と曲構成差し戻し.ts'

// 楽曲編集コマンドの適用を取り消すために必要な変更前情報の直和型。
export type 差し戻し断片 =
    | 打ち込み差し戻し断片
    | 楽曲属性差し戻し断片
    | パターン差し戻し断片
    | コード進行と曲構成差し戻し断片
