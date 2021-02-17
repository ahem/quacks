export type Field = Readonly<{
    value: number;
    victoryPoints: number;
    ruby: boolean;
}>;

export type Color = "white" | "orange" | "green" | "blue" | "red" | "yellow" | "purple" | "black";

export type Chip = Readonly<{
    value: 1 | 2 | 4;
    color: Color;
}>;
