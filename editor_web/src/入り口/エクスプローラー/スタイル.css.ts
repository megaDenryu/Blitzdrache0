import { style } from '@vanilla-extract/css'

// 左サイドバーのエクスプローラー木ビューのスタイル定義。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '8px 0',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: '#858585',
    padding: '6px 12px 2px 12px',
    letterSpacing: '0.5px',
})

export const 木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px',
    fontSize: '13px',
    color: '#cccccc',
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
            backgroundColor: 'rgba(255, 255, 255, 0.15)',
            color: '#ffffff',
            fontWeight: 'bold',
        },
    },
})

export const 子木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px 4px 28px',
    fontSize: '12px',
    color: '#cccccc',
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
            backgroundColor: 'rgba(255, 255, 255, 0.15)',
            color: '#ffffff',
            fontWeight: 'bold',
        },
    },
})

export const アイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: '#3794ff',
})

export const フォルダアイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: '#cca700',
})
