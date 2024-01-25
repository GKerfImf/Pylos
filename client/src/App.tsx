import React from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment } from "@react-three/drei";

import Arena from "../components/arena";

export default function App() {
  return (
    <Canvas shadows camera={{ position: [6, 6, 6] }}>
      <Environment background preset="forest" blur={0.05} />
      <Arena />
      <OrbitControls
        enableDamping={true}
        enableZoom={false}
        dampingFactor={0.15}
      />
    </Canvas>
  );
}
