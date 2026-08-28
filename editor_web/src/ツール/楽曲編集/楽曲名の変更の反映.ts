import type { I楽曲名の欄配線 } from './画面/index.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'

// 楽曲名の欄への入力を、表示名を変えるコマンドとして積む。
// 文書タブの見出しへの反映は、コマンドの適用が通す表示の同期が受け持つ。
export class 楽曲名の変更の反映 implements I楽曲名の欄配線 {
    public constructor(private readonly _操作: 楽曲履歴適用サービス) {}

    public on表示名変更(新しい表示名: string): void {
        this._操作.コマンドを実行する({ 種類: '楽曲の表示名を変える', 値: { 新しい表示名 } })
    }
}
