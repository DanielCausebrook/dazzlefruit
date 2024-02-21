<script lang="ts">
    import * as THREE from "three";
    import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
    import {EffectComposer} from 'three/addons/postprocessing/EffectComposer.js';
    import {UnrealBloomPass} from 'three/addons/postprocessing/UnrealBloomPass.js';
    import {RenderPass} from 'three/addons/postprocessing/RenderPass.js';
    import {OutputPass} from 'three/addons/postprocessing/OutputPass.js';
    import {onMount, tick} from "svelte";
    import type {PositionMap} from "../PatternBuilder.svelte";

    export const initialPositionMap: PositionMap = [];

    let positionMapListeners: [function(PositionMap):void] = [];
    export function updatePositionMap(positionMap: PositionMap) {
        for (const positionMapListener of positionMapListeners) {
            positionMapListener(positionMap);
        }
    }

    export let pixelColorData: [[number, number, number, number]] = [];

    onMount(async () => {
        await tick();

        const canvas = document.querySelector(".df-preview-3d");
        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(75, canvas.clientWidth/canvas.clientHeight, 0.1, 1000);

        const renderer = new THREE.WebGLRenderer({antialias: true});
        renderer.setSize(canvas.clientWidth, canvas.clientHeight);
        canvas.appendChild(renderer.domElement);

        const composer = new EffectComposer( renderer );
        const renderPass = new RenderPass( scene, camera );
        composer.addPass( renderPass );

        const bloomPass = new UnrealBloomPass(new THREE.Vector2( canvas.clientWidth, canvas.clientHeight ), 1, 0.5, 0);
        composer.addPass(bloomPass);

        const outputPass = new OutputPass();
        composer.addPass( outputPass );

        const controls = new OrbitControls( camera, renderer.domElement );
        controls.enablePan = false;
        controls.minPolarAngle = 0.25 * Math.PI;
        controls.maxPolarAngle = 0.75 * Math.PI;
        controls.maxDistance = 5;
        controls.minDistance = 0.2;

        function updateCamera(center: THREE.Vector3, distance: number) {
            controls.target = center;
            controls.minDistance = 5;
            controls.maxDistance = distance * 5;
            camera.position.copy(center).add(new THREE.Vector3(0, 0, distance));
            camera.lookAt(center);
        }

        let lights: [THREE.Mesh|null] = [];

        function updateLights(positionMap: PositionMap) {
            scene.clear();

            let vPositionMap: [THREE.Vector3|null] =
                positionMap.map(pos => pos === null ? null : new THREE.Vector3(pos[0], pos[1], pos[2]));

            lights = vPositionMap.map(pos => {
                if (pos === null) return null;

                const geometry = new THREE.SphereGeometry(0.2);
                const material = new THREE.MeshBasicMaterial({color: 0x000000});
                const light = new THREE.Mesh(geometry, material);
                light.position.copy(pos);
                scene.add(light);
                return light;
            });

            let positions = vPositionMap.filter(pos => pos !== null);

            if (positions.length === 0) {
                updateCamera(new THREE.Vector3(), 1);
                return;
            }

            let center = positions.reduce((sum, pos) => sum.add(pos), new THREE.Vector3()).divideScalar(positions.length);

            let vCenter = center.clone().setX(0).setZ(0);
            let vVariance = positions.reduce((sum, pos) => sum + pos.setX(0).setZ(0).distanceToSquared(vCenter), 0) / positions.length;
            let vStdDev = Math.sqrt(vVariance);

            let hCenter = center.clone().setY(0);
            let hVariance = positions.reduce((sum, pos) => sum + pos.setY(0).distanceToSquared(hCenter), 0) / positions.length;
            let hStdDev = Math.sqrt(hVariance);

            let distance = Math.max(hStdDev, vStdDev) * 2.5;
            updateCamera(center, distance);
        }
        updateLights(initialPositionMap);
        positionMapListeners.push((positionMap: PositionMap) => updateLights(positionMap));

        function animate() {
            requestAnimationFrame(animate);
            camera.aspect = canvas.clientWidth/canvas.clientHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(canvas.clientWidth, canvas.clientHeight);
            bloomPass.setSize(canvas.clientWidth, canvas.clientHeight);

            lights.forEach((light, i) => {
                if (light === null) return;

                let color = 0x000000;
                if (pixelColorData.length > i) {
                    let pixel = pixelColorData[i];
                    let a = pixel[3] / 255;
                    let premult = [pixel[0] * a, pixel[1] * a, pixel[2] * a];
                    color = (premult[0] << 16) + (premult[1] << 8) + premult[2];
                }
                light.material = new THREE.MeshBasicMaterial({color: color});
            })

            composer.render();
        }
        animate();
    });
</script>
<div class="df-preview-3d"></div>
<style lang="scss">
    .df-preview-3d {
      height: 100%;
    }
</style>
