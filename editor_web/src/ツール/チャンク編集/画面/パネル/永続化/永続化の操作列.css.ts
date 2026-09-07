import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 保存・読み込みのボタンと状態文言をひとまとまりの行として並べる。置き場の幅が
// 足りなければ折り返す(置き場の見た目はホスト側の外枠のスタイルで調整する)。
export const 操作列枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

export const アクションボタン = style({
    padding: '6px 12px',
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    color: エディターCSS変数('プライマリボタン文字'),
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    fontSize: '12px',
    fontWeight: 'bold',
    whiteSpace: 'nowrap',
    ':hover': { backgroundColor: エディターCSS変数('プライマリボタンホバー') },
    ':disabled': {
        backgroundColor: エディターCSS変数('非活性背景'),
        color: エディターCSS変数('非活性文字'),
        cursor: 'not-allowed',
    },
})

// 保存(主ボタン)に対する読み込みの副ボタン。枠と文字だけで示す控えめな見た目にする。
export const 副アクションボタン = style({
    padding: '6px 12px',
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('ボタン文字'),
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    fontSize: '12px',
    fontWeight: 500,
    whiteSpace: 'nowrap',
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
    ':disabled': {
        backgroundColor: エディターCSS変数('非活性背景'),
        color: エディターCSS変数('非活性文字'),
        cursor: 'not-allowed',
    },
})

// 状態文言は保存/読込結果やエラーメッセージを表示する1行物のため、末尾を省略記号で
// 収める。全文は呼び出し側がtitle属性で提供する(永続化の操作列.ts の 状態文言を更新する 参照)。
export const 状態メッセージ = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    maxWidth: '160px',
    minHeight: '16px',
})

export const エラー状態メッセージ = style([状態メッセージ, {
    color: エディターCSS変数('危険ボタン文字'),
}])
