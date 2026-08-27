import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const 進行の帯枠 = style({
    display: 'flex',
    alignItems: 'center',
    height: '28px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '4px',
    overflow: 'hidden',
})

export const 進行見出し余白 = style({
    width: '100px',
    minWidth: '100px',
    flexShrink: 0,
    padding: '0 8px',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト薄'),
    boxSizing: 'border-box',
    textAlign: 'right',
})

export const 進行和音列 = style({
    display: 'flex',
    flex: 1,
    height: '100%',
    minWidth: '640px',
})

export const 進行和音ブロック = style({
    height: '100%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '11px',
    fontWeight: 700,
    boxSizing: 'border-box',
    borderRight: `1px solid ${エディターCSS変数('カード枠線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('アクセント文字'),
    selectors: {
        '&[data-repeated="true"]': {
            opacity: 0.65,
            backgroundColor: エディターCSS変数('カード不透明背景'),
            color: エディターCSS変数('テキスト副'),
        },
    },
})
