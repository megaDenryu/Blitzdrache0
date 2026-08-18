import { エディター外殻 } from './入り口/index.ts'

// コンポジションルート。アプリの最外殻コンポーネントを初期化し、#app へマウントする。
function 起動する(): void {
    const アプリ要素 = document.getElementById('app')
    if (アプリ要素 === null) {
        throw new Error('#app が見つからない')
    }

    const 外殻 = new エディター外殻()
    アプリ要素.appendChild(外殻.dom.element)

    const 寸法更新 = (): void => {
        外殻.寸法を合わせる(window.innerWidth, window.innerHeight)
    }
    window.addEventListener('resize', 寸法更新)
    寸法更新()
}

起動する()
