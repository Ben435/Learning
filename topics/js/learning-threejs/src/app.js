const THREE = require('three')

function main() {
    const scene = new THREE.Scene()
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 100)

    const renderer = new THREE.WebGLRenderer({ antialias: true })
    renderer.setSize(window.innerWidth, window.innerHeight)

    document.body.appendChild(renderer.domElement)

    const geometry = new THREE.BoxGeometry(1, 1, 1)
    const material = new THREE.MeshPhongMaterial({ color: 0x00ff00 })
    const cube = new THREE.Mesh(geometry, material)
    scene.add(cube)

    const directionalLight = new THREE.DirectionalLight(0xffffff)
    directionalLight.position.set(-1, 2, 4)
    scene.add(directionalLight)

    camera.position.z = 2

    const startTime = performance.now()
    function animate(delta) {
        requestAnimationFrame(animate)

        const curTime = performance.now()

        cube.rotation.x = Math.sin(curTime / 1000)
        cube.rotation.y = Math.cos(curTime / 1000)

        renderer.render(scene, camera)
    }

    animate(renderer, scene, camera)
}



window.onload = main
