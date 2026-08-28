import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 建物の表示名と識別子を1つの箱へ収める。名前を出すためだけの行を作らないため、
// この箱はエディタ領域の上部の固定の行の左端に並ぶ(設計正本の判断14)。
export const 建物名の枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexShrink: 0,
})

export const 表示名の入力 = style({
    width: '200px',
    padding: '5px 8px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
})

// 保存先を決める識別子。書き換えられないため、入力欄ではなく添えの綴りとして出す。
export const 識別子の添え = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    whiteSpace: 'nowrap',
})
