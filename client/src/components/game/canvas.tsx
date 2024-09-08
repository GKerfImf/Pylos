import React from "react";
import { Canvas } from "@react-three/fiber";
import { EffectComposer, Vignette } from "@react-three/postprocessing";
import { OrbitControls, Environment } from "@react-three/drei";
import Arena from "src/components/game/arena";

// TODO: function --> const
export default function PylosCanvas() {
  return (
    <div className="w-full h-full">
      <Canvas shadows camera={{ position: [-8, 8, 0] }}>
        <Environment background preset="forest" backgroundBlurriness={1} />
        <ambientLight intensity={0.9} />
        <Arena />
        <EffectComposer>
          <Vignette eskil={false} offset={0} darkness={1} />
        </EffectComposer>
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
