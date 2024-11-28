import React, { useEffect, useState } from "react";
import {
  DndContext,
  DragEndEvent,
  useDraggable,
  useDroppable,
} from "@dnd-kit/core";
import { closestCenter } from "@dnd-kit/core";
import { DrawerItem } from "./DrawerItem";
import { ScreenSelector } from "../..";
import { Card, CardContent, Typography } from "@mui/material";

// Define the type for the grid state
type GridState = Record<string, ScreenSelector | null>;

interface DraggableItemProps {
  id: number;
  item: ScreenSelector | null;
}

interface DroppableCellProps {
  id: number;
  item: ScreenSelector | null;
  onDrop?: (id: string) => void;
}

interface InterfaceProps {
  screenSelector: ScreenSelector[] | undefined;
}

export const ScreenDragAndDrop = ({ screenSelector }: InterfaceProps) => {
  const [grid, setGrid] = useState<GridState>({});

  useEffect(() => {
    let result: GridState = {};
    Array.from({ length: 9 }).forEach((_, index) => {
      result = {
        ...result,
        [index]:
          screenSelector?.find((value) => value.screen_no - 1 === index) ??
          null,
      };
    });
    setGrid(result);
  }, [screenSelector]);

  const DraggableItem: React.FC<DraggableItemProps> = ({ id, item }) => {
    const { attributes, listeners, setNodeRef, transform } = useDraggable({
      id,
    });

    const style: React.CSSProperties = {
      transform: `translate3d(${transform?.x ?? 0}px, ${
        transform?.y ?? 0
      }px, 0)`,
      // padding: "25px 50px 25px 50px",
      background: "black",
      borderRadius: "4px",
      cursor: "grab",
      textAlign: "center",
      display: "flex",
      maxWidth: "125px",
      maxHeight: "120px",
      width: "120px",
      height: "100px",
      alignItems: "center",
      justifyContent: "center",
    };

    return (
      <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
        {item?.hostname}
      </div>
    );
  };

  const DroppableCell: React.FC<DroppableCellProps> = ({ id, item }) => {
    const { isOver, setNodeRef } = useDroppable({
      id,
    });

    const style: React.CSSProperties = {
      width: "130px",
      height: "130px",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      background: isOver ? "lightgreen" : "lightgray",
      border: "1px solid black",
      position: "relative",
    };

    return (
      <div ref={setNodeRef} style={style}>
        {item && <DraggableItem id={id} item={item} />}
      </div>
    );
  };

  const handleDragEnd = (event: DragEndEvent): void => {
    const { active, over } = event;
    console.log("event {}", event);
    if (over && active.id !== over.id) {
      setGrid((prevGrid) => {
        const updatedGrid = { ...prevGrid };
        console.log("updatedGrid {}", updatedGrid);
        // Remove the dragged item from its original cell
        const sourceCell = Object.keys(prevGrid).find((key) => {
          const no = prevGrid[key]?.screen_no;
          if (no) return no - 1 === active.id;
        });
        let dataBak: ScreenSelector | null = null;
        if (sourceCell) {
          dataBak = updatedGrid[sourceCell];
          if (dataBak) dataBak.screen_no = parseInt(over.id.toString()) + 1;
          updatedGrid[sourceCell] = null;
        }
        // Place the dragged item in the target cell
        // @ts-ignore
        updatedGrid[over.id] = dataBak;
        return updatedGrid;
      });
    }
  };

  return (
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <DrawerItem />
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(3, 1fr)",
          gap: "5px",
        }}
      >
        {Array.from({ length: 9 }).map((_, cellId) => (
          <DroppableCell key={cellId} id={cellId} item={grid[cellId]} />
        ))}
      </div>
    </DndContext>
  );
};
