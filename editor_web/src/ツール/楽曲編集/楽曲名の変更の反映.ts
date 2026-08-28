import type { 配線ポート } from 'sengen-ui'
import type { I表示名の編集の届け先 } from '../../文書の表示名の編集/index.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { I楽曲の表示名の届け先 } from './表示の同期.ts'

// 楽曲の表示名の編集を、この道具の2つの行き先へ振り分ける届け先。
// 打っている間の見えは文書タブの見出しへ直に届け、入力が決まったときだけコマンドを積む。
// どちらへ何を届けるかの判定は共通の`表示名の編集をまとめる係`が持ち、ここは行き先を知るだけである。
// 参照: `_doc/設計/楽曲エディター.md`「判断13」
export class 楽曲名の変更の反映 implements I表示名の編集の届け先 {
    public constructor(
        private readonly _表示名の届け先: 配線ポート<I楽曲の表示名の届け先>,
        private readonly _操作: 楽曲履歴適用サービス,
    ) {}

    public 表示名の見えを合わせる(表示名: string): void {
        if (this._表示名の届け先.配線済みか) this._表示名の届け先.先.表示名が変わった(表示名)
    }

    public 表示名を変えるコマンドを積む(新しい表示名: string): void {
        this._操作.コマンドを実行する({ 種類: '楽曲の表示名を変える', 値: { 新しい表示名 } })
    }
}
