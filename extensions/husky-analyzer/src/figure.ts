import {
    decode_array,
    decode_memb,
    d_memb_old,
    decode_string,
    decode_opt,
    decode_number,
} from "src/decode/decode";
import type { Point2d } from "src/geom2d/geom2d";
import type Graphics2dProps from "./figure/Graphics2d";
import type Color from "./figure/Color";
import { decode_graphics2d } from "./figure/Graphics2d";
import type PrimitiveValueVisualProps from "./figure/Primitive";
import { decode_primitive_value } from "./figure/Primitive";

export type MutationsFigureProps = {
    kind: "Mutations";
    mutations: MutationFigureProps[];
};

export type MutationFigureProps = {
    name: string;
    before: FigureProps | null;
    after: FigureProps;
    idx: number;
};

export function decode_mutation(data: unknown): MutationFigureProps {
    let name = decode_string(decode_memb(data, "name"));
    let before = decode_opt(decode_memb(data, "before"), decode_figure_props);
    let after = decode_figure_props(decode_memb(data, "after"));
    let idx = decode_number(decode_memb(data, "idx"));
    return { name, before, after, idx };
}

export type PointGroup = {
    points: Point2d[];
    color: Color;
};
export type GalleryProps = { kind: "Gallery" };
export type Plot2dProps = {
    kind: "Plot2d";
    plot_kind: "Scatter";
    groups: PointGroup[];
    xrange: [number, number];
    yrange: [number, number];
};

type FigureProps =
    | GalleryProps
    | Graphics2dProps
    | Plot2dProps
    | PrimitiveValueVisualProps
    | MutationsFigureProps;
export default FigureProps;

export function decode_figure_props(data: unknown): FigureProps {
    let type = d_memb_old(data, "kind", decode_string);
    switch (type) {
        case "Graphics2d":
            return decode_graphics2d(data);
        case "Primitive":
            return {
                kind: "Primitive",
                value: decode_primitive_value(decode_memb(data, "value")),
            };
        case "Mutations":
            return {
                kind: "Mutations",
                mutations: decode_array(
                    decode_memb(data, "mutations"),
                    decode_mutation
                ),
            };
        default:
            console.log("data is ", data);
            throw new Error("Todo");
    }
}