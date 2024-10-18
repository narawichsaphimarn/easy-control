import Konva from "konva";
import React, { useRef, useEffect, useState } from "react";
import { Stage, Layer } from "react-konva";

interface BoardBody {
  children: JSX.Element | JSX.Element[];
  width: number;
  height: number;
  className?: string;
  getScale?: (e: BoardScaleBody) => void;
}

export interface BoardScaleBody {
  width: number;
  height: number;
}

export const Board: React.FC<BoardBody> = (props: BoardBody) => {
  const boardRef = useRef<Konva.Stage>(null);
  const [scale, setScale] = useState<BoardScaleBody>({ width: 0, height: 0 });

  useEffect(() => {
    if (boardRef.current) {
      const width = boardRef.current.width();
      const height = boardRef.current.height();
      let data: BoardScaleBody = {
        width: width,
        height: height,
      };
      setScale(data);
    }
  }, []);

  useEffect(() => {
    if (props.getScale) props.getScale(scale);
  }, [scale, props.getScale]);

  return (
    <Stage ref={boardRef} {...props}>
      <Layer>{props.children}</Layer>
    </Stage>
  );
};
