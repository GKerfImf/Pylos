import React, { useState } from "react";
import { Outlines } from "@react-three/drei";
import _ from "lodash";

function Sphere({
  id,
  onClick,
  color,
  isClicked,
  isClickable,
  position,
  ...props
}: {
  id: any;
  onClick: any;
  color: any;
  isClicked: any;
  isClickable: any;
  position: any;
}) {
  console.debug("Render [Sphere]", id);

  const [hover, setHover] = useState(false);

  const scale = 0.5;

  const onPointerOver = (e: any) => {
    e.stopPropagation();
    if (isClickable) {
      setHover(true);
    }
  };

  const onPointerOut = (e: any) => {
    e.stopPropagation();
    if (isClickable) {
      setHover(false);
    }
  };

  const emissiveIntensity = (isClickable && (hover || isClicked) ? 0.4 : 0) * (color == "white" ? 5 : 1);

  return (
    <mesh
      {...props}
      position={position}
      dispose={null}
      scale={scale}
      onPointerOver={onPointerOver}
      onPointerOut={onPointerOut}
      onClick={onClick}
    >
      <sphereGeometry args={[1, 64, 64]} />
      <meshStandardMaterial
        metalness={0.1}
        roughness={0}
        emissive={"green"}
        emissiveIntensity={emissiveIntensity}
        color={color}
      />
      {isClickable && isClicked && <Outlines screenspace thickness={4} color="#97dd40" />}
    </mesh>
  );
}

const GhostSphere = ({
  id,
  onClick,
  color,
  position,
  ...props
}: {
  id: any;
  onClick: any;
  color: any;
  position: any;
}) => {
  console.debug("Render [GhostSphere]", id);

  const [hover, setHover] = useState(false);

  const scale = 0.5;

  const onPointerOver = (e: any) => {
    e.stopPropagation();
    setHover(true);
  };

  const onPointerOut = (e: any) => {
    e.stopPropagation();
    setHover(false);
  };

  return (
    <mesh
      {...props}
      scale={scale}
      position={position}
      dispose={null}
      onPointerOver={onPointerOver}
      onPointerOut={onPointerOut}
      onClick={onClick}
    >
      <sphereGeometry args={[1, 64, 64]} />
      <meshStandardMaterial transparent metalness={0.1} roughness={0} opacity={hover ? 0.9 : 0.4} color={color} />
    </mesh>
  );
};

function BlackGhostSphere({ id, onClick, position, ...props }: { id: any; onClick: any; position: any }) {
  return <GhostSphere id={id} onClick={onClick} color="black" position={position} {...props} />;
}

function WhiteGhostSphere({ id, onClick, position, ...props }: { id: any; onClick: any; position: any }) {
  return <GhostSphere id={id} onClick={onClick} color="white" position={position} {...props} />;
}

export { Sphere, BlackGhostSphere, WhiteGhostSphere, GhostSphere };
