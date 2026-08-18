import { style, globalStyle } from '@vanilla-extract/css'

export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const グリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '4px',
    padding: '4px',
    backgroundColor: '#020617',
    borderRadius: '8px',
    border: '1px solid #1e293b',
})

export const モードボタン = style({
    padding: '6px 4px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: 'none',
    cursor: 'pointer',
    backgroundColor: 'transparent',
    color: '#94a3b8',
    transition: 'all 0.15s ease',
    ':hover': {
        color: '#f8fafc',
        backgroundColor: '#1e293b',
    },
})

globalStyle(`${モードボタン}[data-selected="true"]`, {
    backgroundColor: '#0891b2',
    color: '#ffffff',
})

export const ヒント枠 = style({
    padding: '8px 12px',
    fontSize: '11px',
    lineHeight: '1.4',
    backgroundColor: '#020617',
    borderRadius: '8px',
    border: '1px solid #1e293b',
    color: '#94a3b8',
})
