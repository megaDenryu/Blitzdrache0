import { style } from '@vanilla-extract/css'

// 左サイドバーのエクスプローラー木ビューのスタイル定義。
// 背景色 #0f172a に対して WCAG AA(4.5:1)以上のコントラスト比を保証する。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '8px 0',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
    backgroundColor: '#0f172a',
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: '#94a3b8',
    padding: '6px 12px 2px 12px',
    letterSpacing: '0.5px',
})

export const 木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px',
    fontSize: '13px',
    color: '#e2e8f0',
    cursor: 'pointer',
    borderRadius: '3px',
    margin: '1px 4px',
    transition: 'background-color 0.1s',
    selectors: {
        '&:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.08)',
            color: '#ffffff',
        },
        '&[data-selected="true"]': {
            backgroundColor: 'rgba(56, 189, 248, 0.18)',
            color: '#38bdf8',
            fontWeight: 'bold',
        },
    },
})

export const 子木項目コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
})

export const 子木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px 4px 28px',
    fontSize: '12px',
    color: '#cbd5e1',
    cursor: 'pointer',
    borderRadius: '3px',
    margin: '1px 4px',
    transition: 'background-color 0.1s',
    selectors: {
        '&:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.08)',
            color: '#ffffff',
        },
        '&[data-selected="true"]': {
            backgroundColor: 'rgba(56, 189, 248, 0.18)',
            color: '#38bdf8',
            fontWeight: 'bold',
        },
    },
})

export const アイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: '#38bdf8',
})

export const フォルダアイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: '#fbbf24',
})
