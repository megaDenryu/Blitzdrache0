import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const 追加ボタン = style({
    alignSelf: 'flex-start',
    padding: '6px 14px',
    fontSize: '12px',
    fontWeight: 'bold',
    color: エディターCSS変数('プライマリボタン文字'),
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    ':hover': {
        backgroundColor: エディターCSS変数('プライマリボタンホバー'),
    },
})

export const 削除ボタン = style({
    padding: '5px 10px',
    fontSize: '11px',
    color: エディターCSS変数('危険ボタン文字'),
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    flexShrink: 0,
    ':hover': {
        backgroundColor: エディターCSS変数('危険ボタンホバー'),
    },
})
