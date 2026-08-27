import { BoxGeometry } from 'three'
import { メッシュ部品, 基本マテリアルを作る } from 'SengenThree'
import { ベイの奥行メートル, ベイの横幅メートル, 升目の原点を求める, 階の高さメートル } from '../../編集モデル/index.ts'
import type { 升目の座標 } from '../../../../生成/編集資源契約.ts'

// いま選んでいる升目を囲む線の枠。ベイ1つぶんの大きさで固定し、選ばれた升目の場所へ移して見せる。
// 枠の色を役割の識別色と別に取るのは、選択が役割の1つではなく、どの升目を触っているかの知らせだからである。
const 選択の枠の色 = 0xfde047

// ベイの寸法ちょうどだと壁の板と重なって縞が出るため、わずかに外へ広げる。
const 枠を外へ広げる倍率 = 1.04

export class 選んだ升目の枠部品 extends メッシュ部品 {
    public constructor() {
        super(
            new BoxGeometry(
                ベイの横幅メートル * 枠を外へ広げる倍率,
                階の高さメートル * 枠を外へ広げる倍率,
                ベイの奥行メートル * 枠を外へ広げる倍率,
            ),
            基本マテリアルを作る({ 色: 選択の枠の色, ワイヤーフレームか: true }),
        )
        this.実体.visible = false
    }

    // 選んでいる升目が無いときは枠を消す。どこかへ残して見せると、選んでいないものを選んで見せることになる。
    public 選んだ升目へ移す(座標: 升目の座標 | undefined): void {
        if (座標 === undefined) {
            this.実体.visible = false
            return
        }
        const 原点 = 升目の原点を求める(座標)
        this.実体.position.set(原点.x, 原点.y + 階の高さメートル / 2, 原点.z)
        this.実体.visible = true
    }
}
