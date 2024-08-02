import { uniqueNamesGenerator, adjectives, colors } from "unique-names-generator";

const generateDefaultName = () => {
  return uniqueNamesGenerator({
    dictionaries: [adjectives, colors],
    length: 2,
    separator: " ",
    style: "capital",
  });
};

export default generateDefaultName;
