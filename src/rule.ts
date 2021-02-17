import type { Cauldron } from "./cauldron";
import type { Chip } from "./types";

export type Rule =
    | {
          type: "chip-value";
          color: Chip["color"];
          apply: (chip: Chip, cauldron: Cauldron) => number;
      }
    | {
          type: "explosion-has-exploded";
          apply: (cauldron: Cauldron) => boolean;
      };

function getRule(rules: Rule[], type: Rule["type"]) {}
