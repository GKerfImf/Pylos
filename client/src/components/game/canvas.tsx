import React, { ReactNode, useCallback, useEffect, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment } from "@react-three/drei";
import Arena from "src/components/game/arena";

import * as THREE from "three";
import {
  ChromaticAberration,
  ColorAverage,
  EffectComposer,
  Noise,
  Pixelation,
  Vignette,
} from "@react-three/postprocessing";
import { BlendFunction } from "postprocessing";
import { Perf } from "r3f-perf";
import { useParams } from "react-router-dom";

// TODO: function --> const
export default function PylosCanvas() {
  let { id } = useParams();

  const [effects, setEffects] = useState(false);

  return (
    <div className="w-full h-full">
      <Canvas shadows camera={{ position: [-8, 8, 0] }}>
        {/* <Perf /> */}
        <Environment background preset="forest" backgroundBlurriness={1} />
        <ambientLight intensity={0.9} />
        <Arena />
        {effects ? (
          <EffectComposer>
            <Vignette eskil={false} offset={0} darkness={1.2} />

            <ColorAverage
              blendFunction={BlendFunction.NORMAL} // blend mode
            />
            <Pixelation granularity={2} />

            <ChromaticAberration
              blendFunction={BlendFunction.NORMAL}
              offset={new THREE.Vector2(0.005, 0.002)}
              radialModulation={false}
              modulationOffset={0}
            />
            <Noise premultiply blendFunction={BlendFunction.AVERAGE} />
          </EffectComposer>
        ) : (
          <EffectComposer>
            <Vignette eskil={false} offset={0} darkness={1} />
          </EffectComposer>
        )}
        <OrbitControls
          enableDamping={true}
          enableZoom={false}
          enablePan={false}
          dampingFactor={0.15}
          maxPolarAngle={Math.PI / 2 - 0.2}
          minPolarAngle={0.2}
          target={[0, 2.121, 0]}
        />
      </Canvas>
    </div>
  );
}
