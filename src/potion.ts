import type { Chip, Color } from "./types";

export type Potion = Readonly<{
    fieldIndex: number;
    chips: Chip[];
}>;

export function addChip(potion: Potion, chip: Chip, value: number): Potion {
    return {
        ...potion,
        fieldIndex: potion.fieldIndex + value,
        chips: [...potion.chips, chip],
    };
}

export function totalValue(potion: Potion, color: Color): number {
    return potion.chips
        .filter((chip) => chip.color === color)
        .reduce((acc, chip) => acc + chip.value, 0);
}

export function count(potion: Potion, color: Color): number {
    return potion.chips.filter((chip) => chip.color === color).length;
}

export function lastChip(potion: Potion): Chip {
    return potion.chips[potion.chips.length - 1];
}
