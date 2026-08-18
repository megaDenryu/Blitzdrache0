import { GridHelper } from 'three'
import { 三次元部品 } from 'SengenThree'

// 1024m四方における4x4チャンク境界の格子線を表示する部品。
export class チャンク境界格子部品 extends 三次元部品<GridHelper> {
    public constructor(世界一辺: number = 1024, 軸あたり分割数: number = 4) {
        const helper = new GridHelper(世界一辺, 軸あたり分割数, 0x10b981, 0x334155)
        helper.position.y = 1.0
        super(helper)
        this.資源台帳.登録する(helper.geometry)
        if (Array.isArray(helper.material)) {
            for (const m of helper.material) {
                this.資源台帳.登録する(m)
            }
        } else {
            this.資源台帳.登録する(helper.material)
        }
    }
}
