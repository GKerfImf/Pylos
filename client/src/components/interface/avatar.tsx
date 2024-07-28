import React from "react";
import BoringAvatar from "boring-avatars";
import { useParams } from "react-router-dom";

// Icons downloaded with a premium account
import crownImage1 from "../../icons/crown1.svg"; // https://www.flaticon.com/free-icon/crown_7006900
import crownImage2 from "../../icons/crown2.svg"; // https://www.flaticon.com/free-icon/crown_9028075
import crownImage3 from "../../icons/crown3.svg"; // https://www.flaticon.com/free-icon/crown_2460546

const Crown: React.FC<{ id: string }> = ({ id }) => {
  const crownImages = [crownImage1, crownImage2, crownImage3];

  let { id: url } = useParams(); // For additional randomness
  const crownIndex =
    (id + url)
      .split("")
      .map((c) => c.charCodeAt(0))
      .reduce((acc: number, v: number) => {
        return acc + v;
      }, 0) % crownImages.length;

  const src = crownImages[crownIndex];

  return (
    <img className="absolute -top-[32px] left-[6px] z-50 h-10 rotate-12" src={src} alt="Description of the image" />
  );
};

const Avatar: React.FC<{ id: string; winner: boolean }> = ({ id, winner }) => {
  return (
    <div className="relative">
      {winner ? <Crown id={id} /> : null}
      <BoringAvatar name={id} colors={["#ffabab", "#ffdaab", "#ddffab", "#abe4ff", "#d9abff"]} variant="beam" />
    </div>
  );
};

export default Avatar;
