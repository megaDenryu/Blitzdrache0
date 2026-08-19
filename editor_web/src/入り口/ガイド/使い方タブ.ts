import { div, span, p, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 使い方の閲覧記録を書く } from '../../境界/index.ts'
import { コンテナ, 本文幅, 表題, セクション, セクション見出し, 段落, 手順リスト, コード欄 } from './スタイル.css.ts'

const ヘッドレス実行コマンド = 'node --experimental-transform-types src/ヘッドレス/編集コマンド一括適用.ts <コマンドJSONのパス>'

// エディタエリアに開く使い方ガイドのタブ内容。三次元ビューを持たない静的ページであり、
// 実行可能ツールの契約(寸法を合わせる・前面/背面になった)は空実装で満たす。
export class 使い方タブ extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor() {
        super()
        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                div({ class: 表題, text: 'このエディターの使い方' }),
                this._この道具は何か(),
                this._基本の流れ(),
                this._三次元ビューの操作(),
                this._AIからの操作(),
                this._テーマの切替(),
            ]),
        )
    }

    public 寸法を合わせる(): void {}

    // 実際に前面へ出て見られたときに閲覧済みとして記録する(構築しただけでは記録しない)。
    public 前面になった(): void {
        使い方の閲覧記録を書く()
    }

    public 背面になった(): void {}

    private _この道具は何か(): DivC {
        return div({ class: セクション }).childs([
            div({ class: セクション見出し, text: 'この道具は何か' }),
            p({
                class: 段落,
                text: 'このエディターは、大域編集とチャンク編集の2つの道具を持つ。大域編集は、世界の区画割り・マザーハイトマップ(世界全体を1枚でつなげた高さ場)・広域道路を編集する道具である。チャンク編集は、1つのチャンクの高さ場・地表材質・道路・建物・散布を編集する道具である。左のエクスプローラーで対象を開くと、開いた対象ごとにエディタエリアへタブが1枚開く。',
            }),
        ])
    }

    private _基本の流れ(): DivC {
        return div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '基本の流れ' }),
            div({ class: 手順リスト }).childs([
                p({ text: '1. 左のエクスプローラーで、編集したい対象(大域世界またはチャンク)を開く。' }),
                p({ text: '2. モード・ブラシ・道路・建物の各パネルで編集する。' }),
                p({ text: '3. 操作を間違えたらCtrl+Zで取り消す。' }),
                p({ text: '4. インスペクターの保存ボタンで保管庫へ保存し、読み込みボタンで保管庫から読み直す。' }),
                p({ text: '5. 大域編集の「チャンク切り出しの設定」パネルの書き出しボタンで、保存済みの内容をソースアセットへ書き出す。' }),
            ]),
        ])
    }

    private _三次元ビューの操作(): DivC {
        return div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '三次元ビューの操作' }),
            div({ class: 手順リスト }).childs([
                p({ text: '右ドラッグで視点回転、中ドラッグで平行移動、ホイールでズーム(全モード共通)。' }),
                p({ text: '左ドラッグは選んでいるモードの主作業になる(選択モードでは何も起きない)。' }),
                p({ text: 'W/Sで注視点ごと前後移動、A/Dで左右移動、Eで上昇、Qで下降。押している間だけ動き、速度はズームの距離に比例する。' }),
                p({ text: 'Altでモードを次へ、Shift+Altで前へ切り替える。入力欄にフォーカスしている間はどちらも無効。' }),
            ]),
        ])
    }

    private _AIからの操作(): DivC {
        return div({ class: セクション }).childs([
            div({ class: セクション見出し, text: 'AIからの操作' }),
            p({
                class: 段落,
                text: 'AIは画面を持たずにこのエディターを操作できる。編集コマンドの列をJSONファイルへ書き出し、editor_webディレクトリで次の1行を実行すると、人がボタンやブラシで操作したのと同じ適用実装が同じコマンドを適用する。画面を持つ入り口と画面を持たない入り口は、同じ編集モデルと同じ操作コマンドの適用処理を呼んでいるだけであり、意味の異なる別実装ではない。',
            }),
            span({ class: コード欄, text: ヘッドレス実行コマンド }),
        ])
    }

    private _テーマの切替(): DivC {
        return div({ class: セクション }).childs([
            div({ class: セクション見出し, text: 'テーマの切替' }),
            p({
                class: 段落,
                text: '左のアクティビティバー下部の歯車アイコンから設定タブを開くと、画面全体のカラーテーマを切り替えられる。選んだテーマは次回起動時も復元される。',
            }),
        ])
    }
}
