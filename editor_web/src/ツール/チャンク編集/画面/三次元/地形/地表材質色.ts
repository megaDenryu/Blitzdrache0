// 地表材質4層(草・泥・岩・砂)それぞれの識別色。"#rrggbb"形式の文字列で持ち、
// three.jsのColorコンストラクタへそのまま渡せる(マテリアル台帳の識別色と同じ書式)。
export interface 地表材質色 {
    readonly 草: string
    readonly 泥: string
    readonly 岩: string
    readonly 砂: string
}
