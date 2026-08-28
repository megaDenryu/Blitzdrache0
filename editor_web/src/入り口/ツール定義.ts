import type { HtmlComponentBase } from 'sengen-ui'
import type { エディターテーマ定義 } from './テーマ/index.ts'

// エディタタブに配置され寸法調整・インスペクター提供・下パネル提供・前面背面ライフサイクル・
// テーマ適用を持つツールの仕様。
//
// インスペクターと下パネルはどちらも任意である。インスペクターは「選んだものをどう設定するか」を
// 右サイドバーへ、下パネルは「これから何を配置・利用するか」の棚を画面下へ出す。
// 下パネルを持たないツールでは、外殻が下パネルそのものを閉じる。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export interface 実行可能ツール {
    寸法を合わせる(幅: number, 高さ: number): void
    前面になった(): void
    背面になった(): void
    delete(): void
    readonly インスペクター?: HtmlComponentBase
    readonly 下パネル?: HtmlComponentBase
    テーマを適用する?(テーマ: エディターテーマ定義): void
}
