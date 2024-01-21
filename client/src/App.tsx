import React from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment, ContactShadows } from "@react-three/drei";
import { Model } from "./Shoe";

export default function App() {
  return (
    <Canvas shadows camera={{ position: [1, 1, 1] }}>
      <Environment preset="forest" />
      <Model />
      <ContactShadows position={[0, 0, 0]} color="#ffffff" />
      <OrbitControls autoRotate />
    </Canvas>
  );
}
