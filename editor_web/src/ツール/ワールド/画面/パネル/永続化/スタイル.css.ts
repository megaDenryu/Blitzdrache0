import { style } from '@vanilla-extract/css'

export const 永続化枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: '#1e1e1e',
    borderRadius: '4px',
    border: '1px solid #333',
    marginTop: '8px',
})

export const ボタン行 = style({
    display: 'flex',
    gap: '8px',
})

export const アクションボタン = style({
    flex: 1,
    padding: '6px 12px',
    backgroundColor: '#0e639c',
    color: '#ffffff',
    border: 'none',
    borderRadius: '3px',
    cursor: 'pointer',
    fontSize: '12px',
    fontWeight: 'bold',
    ':hover': {
        backgroundColor: '#1177bb',
    },
    ':disabled': {
        backgroundColor: '#444444',
        color: '#888888',
        cursor: 'not-allowed',
    },
})

export const 状態メッセージ = style({
    fontSize: '11px',
    color: '#cccccc',
    wordBreak: 'break-all',
    minHeight: '16px',
})

export const エラー状態メッセージ = style({
    fontSize: '11px',
    color: '#f48771',
    wordBreak: 'break-all',
    minHeight: '16px',
})
