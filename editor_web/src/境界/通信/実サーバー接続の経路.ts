import type { チャンク座標 } from '../../生成/編集資源契約.ts'

export type 大域世界資源区分 = '構造' | '高さ格子'
export type チャンク資源区分 = '構造' | '高さ格子' | '材質重み'

// 実サーバー接続が使うAPI経路の綴りをこの1箇所へ集約する純粋関数群
// (自由関数の許容2条件(a): 依存も副作用も持たない。参照: CLAUDE.md「切り出しの根拠義務」第5項)。
export function 大域世界パスを組み立てる(基底URL: string, 区分: 大域世界資源区分): string {
    return `${基底URL}/api/大域世界/${区分}`
}

export function チャンクパスを組み立てる(基底URL: string, 座標: チャンク座標, 区分: チャンク資源区分): string {
    return `${基底URL}/api/チャンク/${座標.x}/${座標.z}/${区分}`
}

export function 書き出しパスを組み立てる(基底URL: string): string {
    return `${基底URL}/api/書き出し/ソースアセット`
}

export function 建物外形カタログパスを組み立てる(基底URL: string): string {
    return `${基底URL}/api/建物外形カタログ`
}
