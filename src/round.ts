import type { Cauldron } from "./cauldron";
import type { Bag } from "./bag";
import type { Rule } from "./rule";
import type { Chip } from "./types";

import { draw } from "./bag";
import { addChip } from "./cauldron";

export type Action = { type: "draw" | "stop" };

/*
 * - draw chip
 * - resolve chip value [check rules]
 * - add chip to pot
 * - extra chip action [check rules]
 * - check for explosion [check rules]
 * - stop or draw
 */

type Player = {
    bag: Bag;
    dropPosition: number;
    ratAdvantage: number;
    shouldDrawNext: (cauldron: Cauldron) => boolean
};

export function fillCauldron(player: Player, rules: Rule[]) {
    let cauldron: Cauldron = { fieldIndex: player.dropPosition + player.ratAdvantage, chips: [] };
    let bag = player.bag;
    let chip: Chip;

    while (true) {
        [bag, chip] = draw(bag);

        const value =
            rules
                .find((x) => x.type === "chip-value" && x.color === chip.color)
                ?.apply(chip, cauldron) ?? chip.value;

        cauldron = addChip(cauldron, chip, value);

        const explosionLimit = rules.

        if (!player.shouldDrawNext(cauldron)) {
            break;
        }
    }

    return cauldron;
}
