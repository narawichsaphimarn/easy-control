import React, {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useState,
} from "react";
import {
  closestCenter,
  DndContext,
  DragEndEvent,
  useDroppable,
} from "@dnd-kit/core";
import { DrawerItem, System } from "./DrawerItem";
import { ScreenMatrixRequest } from "../..";
import { DraggableItem } from "./DraggableItem";
import { IconButton, Tooltip } from "@mui/material";
import AddCircleIcon from "@mui/icons-material/AddCircle";
import DeleteIcon from "@mui/icons-material/Delete";
import { red } from "@mui/material/colors";

// Define the type for the grid state
type GridState = Record<string, ScreenMatrixRequest | null>;

interface DroppableCellProps {
  id: number;
  item: ScreenMatrixRequest | null;
  onDrop?: (id: string) => void;
}

interface DroppableTrashProps {
  id: number;
}

interface InterfaceProps {
  screens: ScreenMatrixRequest[] | undefined;
  setScreenMatrix: React.Dispatch<React.SetStateAction<ScreenMatrixRequest[]>>;
}

export type DragChildRef = {
  reset: () => void;
};

export const ScreenDragAndDrop = forwardRef<DragChildRef, InterfaceProps>(
  ({ screens, setScreenMatrix }: InterfaceProps, ref) => {
    const [grid, setGrid] = useState<GridState>({});
    const [open, setOpen] = useState<number | null>(null);
    const [systemBak, setSystemBak] = useState<System[]>([]);
    const [system, setSystem] = useState<System[]>([]);

    const addScreenMatrix = (data: ScreenMatrixRequest) => {
      setScreenMatrix((prevScreen) => [...prevScreen, data]);
    };

    useImperativeHandle(ref, () => ({
      reset() {
        setSystem(systemBak);
        setSystemBak([]);
      },
    }));

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
        background: isOver ? "#FF2929" : "#EBEAFF",
        border: isOver ? "1px solid red" : "1px solid black",
        borderRadius: "4px",
        position: "relative",
      };

      return (
        <div ref={setNodeRef} style={style}>
          {item ? (
            <DraggableItem id={id} item={item} />
          ) : (
            <IconButton
              color="secondary"
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

    const DroppableTrash: React.FC<DroppableTrashProps> = ({ id }) => {
      const { isOver, setNodeRef } = useDroppable({
        id,
      });

      const style: React.CSSProperties = {
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        position: "absolute",
        right: -10,
        bottom: 45,
      };

      const styleIcon: React.CSSProperties = {
        color: isOver ? red[500] : "gray",
      };

      return (
        <div ref={setNodeRef} style={style}>
          <Tooltip title="Delete">
            <DeleteIcon fontSize="large" sx={styleIcon} />
          </Tooltip>
        </div>
      );
    };

    const handleDragEnd = (event: DragEndEvent): void => {
      const { active, over } = event;
      console.log("event {}", event);
      if (over && active.id !== over.id) {
        setGrid((prevGrid) => {
          const updatedGrid = { ...prevGrid };
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
            if (over.id !== 999) {
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
            } else {
              setSystemBak((prevSystem) => {
                return [...prevSystem]
                  .filter((item) => item.mac !== dataBak?.machine.mac)
                  .sort();
              });
              setSystem((prevSystem) => {
                if (dataBak) {
                  return [
                    ...prevSystem,
                    {
                      host_name: dataBak.machine.host_name,
                      ip: dataBak.machine.ip,
                      mac: dataBak.machine.mac,
                      screen: dataBak.machine.screen,
                    },
                  ].sort();
                } else {
                  return prevSystem.sort();
                }
              });
              setScreenMatrix((prevMatrix) => {
                if (dataBak) {
                  let updateMatrix = [...prevMatrix].filter(
                    (item) => item.machine.mac !== dataBak?.machine.mac
                  );
                  return updateMatrix.sort();
                } else {
                  return [...prevMatrix].sort();
                }
              });
            }
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
          setSystemBak={setSystemBak}
          setSystem={setSystem}
          system={system}
          screens={screens ? screens : []}
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
          <DroppableTrash id={999} />
        </div>
      </DndContext>
    );
  }
);
