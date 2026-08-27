import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const トラック枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '6px',
    padding: '10px 12px',
})

export const トラックヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    flexWrap: 'wrap',
    gap: '8px',
    paddingBottom: '4px',
})

export const トラック名 = style({
    fontSize: '13px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト主'),
})

export const トラック属性群 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    flexWrap: 'wrap',
})

export const 属性バッジ = style({
    fontSize: '11px',
    padding: '2px 6px',
    borderRadius: '3px',
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線薄')}`,
    color: エディターCSS変数('テキスト副'),
})
