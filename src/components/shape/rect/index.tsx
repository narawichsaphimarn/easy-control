import { Rect } from "react-konva";
import { ShapeBody } from "../struct";

export const ShapeRect: React.FC<ShapeBody> = (props: ShapeBody) => {
  return <Rect {...props} />;
};
