import { style } from '@vanilla-extract/css'

export const インスペクター枠 = style({
    position: 'absolute',
    top: '16px',
    left: '16px',
    zIndex: 10,
    width: '380px',
    maxHeight: 'calc(100vh - 32px)',
    overflowY: 'auto',
    backgroundColor: 'rgba(15, 23, 42, 0.95)',
    backdropFilter: 'blur(12px)',
    border: '1px solid #1e293b',
    borderRadius: '12px',
    padding: '16px',
    boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)',
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    color: '#e2e8f0',
})

export const ヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    borderBottom: '1px solid #1e293b',
    paddingBottom: '10px',
})

export const タイトル = style({
    fontSize: '13px',
    fontWeight: 700,
    color: '#22d3ee',
    letterSpacing: '0.05em',
})

export const サブタイトル = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: '#94a3b8',
})

export const バッジ = style({
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    fontFamily: 'monospace',
    backgroundColor: '#083344',
    color: '#22d3ee',
    border: '1px solid #155e75',
})
