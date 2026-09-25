import { style, type StyleRule } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 永続化の操作列は2つの置き場で使う。右サイドバーの永続化パネル(チャンク編集・建物編集・マテリアル)では、
// ボタン2つが行の幅を分け合い、状態文言をその下の行へ幅いっぱいに出す。楽曲名の欄では名前の行へ1行で詰め、
// 行を押し広げないよう状態文言の幅に上限を置く。置き場ごとの違いはこのファイルの2組の見た目に閉じる。

const 押しどころの共通: StyleRule = {
    padding: '6px 12px',
    borderRadius: '3px',
    cursor: 'pointer',
    fontSize: '12px',
    whiteSpace: 'nowrap',
    ':disabled': {
        backgroundColor: エディターCSS変数('非活性背景'),
        color: エディターCSS変数('非活性文字'),
        cursor: 'not-allowed',
    },
}

const 保存ボタンの共通 = style({
    ...押しどころの共通,
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    color: エディターCSS変数('プライマリボタン文字'),
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    fontWeight: 'bold',
    ':hover': { backgroundColor: エディターCSS変数('プライマリボタンホバー') },
})

// 保存(主ボタン)に対する読み込みの副ボタン。枠と文字だけで示す控えめな見た目にする。
const 読込ボタンの共通 = style({
    ...押しどころの共通,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('ボタン文字'),
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    fontWeight: 500,
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
})

// 状態文言は保存/読込結果やエラーメッセージを表示する1行物のため、末尾を省略記号で収める。
// 全文は永続化の操作列の 状態文言を更新する がtitle属性で出す。
const 状態文言の共通 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    minHeight: '16px',
})
const エラーの色: StyleRule = { color: エディターCSS変数('危険ボタン文字') }

// サイドバーの幅を2つのボタンで分け合うため、ボタンは伸び、はみ出す文字は省略記号で収める。
const 幅を分け合う: StyleRule = { flex: 1, overflow: 'hidden', textOverflow: 'ellipsis' }

// 永続化の操作列の各部へ当てるクラスの組。置き場ごとに1組を選んで作る。
export interface 永続化の操作列の見た目 {
    readonly 枠: string
    readonly ボタン行: string
    readonly 保存ボタン: string
    readonly 読込ボタン: string
    readonly 状態文言: string
    readonly エラーの状態文言: string
}

export const サイドバーの縦積みの見た目: 永続化の操作列の見た目 = {
    枠: style({ display: 'flex', flexDirection: 'column', gap: '8px' }),
    ボタン行: style({ display: 'flex', gap: '8px' }),
    保存ボタン: style([保存ボタンの共通, 幅を分け合う]),
    読込ボタン: style([読込ボタンの共通, 幅を分け合う]),
    状態文言: 状態文言の共通,
    エラーの状態文言: style([状態文言の共通, エラーの色]),
}

export const 名前の行の横並びの見た目: 永続化の操作列の見た目 = {
    枠: style({ display: 'flex', alignItems: 'center', gap: '8px' }),
    ボタン行: style({ display: 'flex', gap: '8px' }),
    保存ボタン: 保存ボタンの共通,
    読込ボタン: 読込ボタンの共通,
    状態文言: style([状態文言の共通, { maxWidth: '160px' }]),
    エラーの状態文言: style([状態文言の共通, エラーの色, { maxWidth: '160px' }]),
}
