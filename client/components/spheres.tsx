import React, { useState } from "react";
import { Outlines } from "@react-three/drei";

type SphereProps = {
  color: string;
  isClicked: boolean;
  isClickable: boolean;
  signalClick: () => void;
};

function Sphere({
  color,
  isClicked,
  isClickable,
  signalClick,
  ...props
}: SphereProps & any) {
  const [hover, setHover] = useState(false);

  const emissiveIntensity =
    (isClickable && (hover || isClicked) ? 0.2 : 0) *
    (color == "white" ? 8 : 1);

  return (
    <mesh
      {...props}
      dispose={null}
      scale={0.5}
      onPointerOver={(e) => {
        e.stopPropagation();
        if (isClickable) {
          setHover(true);
        }
      }}
      onPointerOut={(e) => {
        e.stopPropagation();
        if (isClickable) {
          setHover(false);
        }
      }}
      onClick={(e) => {
        e.stopPropagation();
        if (isClickable) {
          signalClick();
        }
      }}
    >
      <sphereGeometry args={[1, 64, 64]} />
      <meshStandardMaterial
        metalness={0.1}
        roughness={0}
        emissive={"green"}
        emissiveIntensity={emissiveIntensity}
        color={color}
      />
      {isClickable && isClicked && (
        <Outlines screenspace thickness={4} color="#97dd40" />
      )}
    </mesh>
  );
}

type GhostSphereProps = {
  color: string;
  // isClicked: boolean;
  // isClickable: boolean;
  signalClick: () => void;
};

const GhostSphere = ({
  color,
  signalClick,
  ...props
}: GhostSphereProps & any) => {
  const [hover, setHover] = useState(false);

  return (
    <mesh
      {...props}
      scale={0.5}
      dispose={null}
      onPointerOver={(e) => {
        e.stopPropagation();
        setHover(true);
      }}
      onPointerOut={(e) => {
        e.stopPropagation();
        setHover(false);
      }}
      onClick={(e) => {
        e.stopPropagation();
        signalClick();
      }}
    >
      <sphereGeometry args={[1, 64, 64]} />
      <meshStandardMaterial
        transparent
        metalness={0.1}
        roughness={0}
        opacity={hover ? 0.8 : 0.4}
        color={color}
      />
    </mesh>
  );
};

function BlackSphere({ isClicked, isClickable, signalClick, ...props }) {
  return (
    <Sphere
      isClicked={isClicked}
      isClickable={isClickable}
      signalClick={signalClick}
      color="black"
      {...props}
    />
  );
}
function WhiteSphere({ isClicked, isClickable, signalClick, ...props }) {
  return (
    <Sphere
      isClicked={isClicked}
      isClickable={isClickable}
      signalClick={signalClick}
      color="white"
      {...props}
    />
  );
}

// function WhiteSphere({ ...props }) {
//   return <Sphere color="white" {...props} />;
// }

function BlackGhostSphere({ signalClick, ...props }) {
  return <GhostSphere color="black" signalClick={signalClick} {...props} />;
}

function WhiteGhostSphere({ signalClick, ...props }) {
  return <GhostSphere color="white" signalClick={signalClick} {...props} />;
}

export {
  BlackSphere,
  WhiteSphere,
  Sphere,
  BlackGhostSphere,
  WhiteGhostSphere,
  GhostSphere,
};
