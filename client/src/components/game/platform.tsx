//@ts-nocheck
import React from "react";
import { useGLTF } from "@react-three/drei";

export default function Platform() {
  const { nodes, materials } = useGLTF("/models/pylos.glb");
  return (
    <group dispose={null}>
      <mesh castShadow receiveShadow geometry={nodes.Platform.geometry} material={materials.M_plaftorm}></mesh>
      <mesh
        castShadow
        receiveShadow
        geometry={nodes.Platform_W.geometry}
        material={materials.M_platform_W}
        position={[0, 0, 4.5]}
      />
      <mesh
        castShadow
        receiveShadow
        geometry={nodes.Platform_B.geometry}
        material={materials.M_platform_B}
        position={[0, 0, -4.5]}
      />
    </group>
  );
}
useGLTF.preload("/models/pylos.glb");
