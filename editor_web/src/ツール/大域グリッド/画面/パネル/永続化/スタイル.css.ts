import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const 永続化枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    marginTop: '8px',
})

export const ボタン行 = style({
    display: 'flex',
    gap: '8px',
})

export const アクションボタン = style({
    flex: 1,
    padding: '6px 12px',
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    color: エディターCSS変数('プライマリボタン文字'),
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    fontSize: '12px',
    fontWeight: 'bold',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('プライマリボタンホバー'),
    },
    ':disabled': {
        backgroundColor: エディターCSS変数('非活性背景'),
        color: エディターCSS変数('非活性文字'),
        cursor: 'not-allowed',
    },
})

// 状態文言は保存/読込結果やエラーメッセージを表示する1行物のため、任意の位置で
// 折り返す(word-break: break-all)のではなく、末尾を省略記号で収める。全文は
// 呼び出し側がtitle属性で提供する(大域永続化パネル.ts の 状態文言を更新する 参照)。
export const 状態メッセージ = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    minHeight: '16px',
})

export const エラー状態メッセージ = style({
    fontSize: '11px',
    color: エディターCSS変数('危険ボタン文字'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    minHeight: '16px',
})
