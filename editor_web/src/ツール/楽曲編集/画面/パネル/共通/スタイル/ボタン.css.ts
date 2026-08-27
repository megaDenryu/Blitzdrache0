import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

export const 主ボタン = style({
    padding: '6px 12px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    color: エディターCSS変数('プライマリボタン文字'),
    cursor: 'pointer',
    fontWeight: 600,
    ':hover': { backgroundColor: エディターCSS変数('プライマリボタンホバー') },
    ':disabled': { opacity: 0.5, cursor: 'not-allowed' },
})

export const 副ボタン = style({
    padding: '4px 10px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('ボタン文字'),
    cursor: 'pointer',
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
    ':disabled': { opacity: 0.5, cursor: 'not-allowed' },
})

export const 危険ボタン = style({
    padding: '4px 10px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    color: エディターCSS変数('危険ボタン文字'),
    cursor: 'pointer',
    ':hover': { backgroundColor: エディターCSS変数('危険ボタンホバー') },
    ':disabled': { opacity: 0.4, cursor: 'not-allowed' },
})

// 記号1文字だけを載せる小さなボタン。行の中で場所を取らないよう余白を詰める。
export const 記号ボタン = style({
    padding: '3px 8px',
    fontSize: '11px',
    borderRadius: '3px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('ボタン文字'),
    cursor: 'pointer',
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
    ':disabled': { opacity: 0.3, cursor: 'not-allowed' },
})

export const 危険な記号ボタン = style([記号ボタン, {
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    color: エディターCSS変数('危険ボタン文字'),
    ':hover': { backgroundColor: エディターCSS変数('危険ボタンホバー') },
}])
