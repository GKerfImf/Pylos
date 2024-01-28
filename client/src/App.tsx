import React from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment, Stats } from "@react-three/drei";
import Arena from "../components/arena";

export default function App() {
  return (
    <Canvas shadows camera={{ position: [-8, 8, 0] }}>
      <Stats />
      <Environment background preset="forest" blur={1} />
      <ambientLight intensity={0.5} />
      <Arena />
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
  );
}
