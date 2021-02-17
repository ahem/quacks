import type { Chip, Color } from "./types";

export type Cauldron = Readonly<{
    fieldIndex: number;
    chips: Chip[];
}>;

export const emptyPotion = {};

export function addChip(cauldron: Cauldron, chip: Chip, value: number): Cauldron {
    return {
        ...cauldron,
        fieldIndex: cauldron.fieldIndex + value,
        chips: [...cauldron.chips, chip],
    };
}

export function totalValue(cauldron: Cauldron, color: Color): number {
    return cauldron.chips
        .filter((chip) => chip.color === color)
        .reduce((acc, chip) => acc + chip.value, 0);
}

export function count(cauldron: Cauldron, color: Color): number {
    return cauldron.chips.filter((chip) => chip.color === color).length;
}

export function lastChip(cauldron: Cauldron): Chip {
    return cauldron.chips[cauldron.chips.length - 1];
}
