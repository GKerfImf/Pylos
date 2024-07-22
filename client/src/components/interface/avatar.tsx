import React, { useEffect } from "react";
import BoringAvatar from "boring-avatars";

const Avatar: React.FC<{ id: string }> = ({ id }) => {
  return <BoringAvatar name={id} colors={["#ffabab", "#ffdaab", "#ddffab", "#abe4ff", "#d9abff"]} variant="beam" />;
};

export default Avatar;
