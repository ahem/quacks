import type { Chip, Color } from "./types";

export type Bag = {
    chips: Chip[];
};

export class EmptyBagException extends Error {}

export function draw(bag: Bag): [Bag, Chip] {
    const idx = Math.floor(Math.random() * bag.chips.length);
    const drawn = bag.chips[idx];
    if (!drawn) {
        throw new EmptyBagException();
    }
    return [{ ...bag, chips: bag.chips.filter((x) => x !== drawn) }, drawn];
}

export function oddsToDraw(bag: Bag, color: Color) {
    const cnt = bag.chips.filter((x) => x.color === color).length;
    return cnt / bag.chips.length;
}
