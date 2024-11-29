import React, { useEffect, useState } from "react";
import { DndContext, DragEndEvent, useDroppable } from "@dnd-kit/core";
import { closestCenter } from "@dnd-kit/core";
import { DrawerItem } from "./DrawerItem";
import { ScreenMatrixRequest } from "../..";
import { DraggableItem } from "./DraggableItem";
import { IconButton } from "@mui/material";
import AddCircleIcon from "@mui/icons-material/AddCircle";

// Define the type for the grid state
type GridState = Record<string, ScreenMatrixRequest | null>;

interface DroppableCellProps {
  id: number;
  item: ScreenMatrixRequest | null;
  onDrop?: (id: string) => void;
}

interface InterfaceProps {
  screens: ScreenMatrixRequest[] | undefined;
  setScreenMatrix: React.Dispatch<React.SetStateAction<ScreenMatrixRequest[]>>;
}

export const ScreenDragAndDrop = ({
  screens,
  setScreenMatrix,
}: InterfaceProps) => {
  const [grid, setGrid] = useState<GridState>({});
  const [open, setOpen] = useState<number | null>(null);

  const addScreenMatrix = (data: ScreenMatrixRequest) => {
    setScreenMatrix((prevScreen) => [...prevScreen, data]);
  };

  useEffect(() => {
    let result: GridState = {};
    Array.from({ length: 9 }).forEach((_, index) => {
      result = {
        ...result,
        [index]:
          screens?.find((value) => value.screen_no - 1 === index) ?? null,
      };
    });
    setGrid(result);
  }, [screens]);

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
      background: "lightgray",
      border: isOver ? "3px solid red" : "1px solid black",
      borderRadius: "4px",
      position: "relative",
    };

    return (
      <div ref={setNodeRef} style={style}>
        {item ? (
          <DraggableItem id={id} item={item} />
        ) : (
          <IconButton
            color="primary"
            aria-label="add"
            onClick={() => {
              setOpen(id + 1);
            }}
          >
            <AddCircleIcon fontSize="large" />
          </IconButton>
        )}
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
        let dataBak: ScreenMatrixRequest | null = null;
        if (sourceCell) {
          dataBak = updatedGrid[sourceCell];
          if (dataBak) {
            dataBak.screen_no = parseInt(over.id.toString()) + 1;
          }
          updatedGrid[sourceCell] = null;
          // Place the dragged item in the target cell
          updatedGrid[over.id] = dataBak;
          setScreenMatrix((prevMatrix) => {
            if (dataBak) {
              let updateMatrix = [...prevMatrix].filter(
                (item) => item.machine.mac !== dataBak?.machine.mac
              );
              return [...updateMatrix, dataBak].sort();
            } else {
              return [...prevMatrix];
            }
          });
        }
        return updatedGrid;
      });
    }
  };

  return (
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <DrawerItem
        isOpen={open}
        setIsOpen={setOpen}
        addScreenMatrix={addScreenMatrix}
      />
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
