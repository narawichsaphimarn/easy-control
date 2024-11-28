import React, {useState} from "react";
import {DndContext, DragEndEvent, useDraggable, useDroppable} from "@dnd-kit/core";
import { closestCenter } from '@dnd-kit/core';

// Define the type for the grid state
type GridState = Record<string, string | null>;

const initialGrid: string[][] = Array(3)
    .fill(null)
    .map((_, rowIndex) =>
        Array(3)
            .fill(null)
            .map((_, colIndex) => `${rowIndex}-${colIndex}`)
    );

interface DraggableItemProps {
    id: string;
}

interface DroppableCellProps {
    id: string;
    item: string | null;
    onDrop?: (id: string) => void;
}

export const ScreenDragAndDrop = () => {
    const [grid, setGrid] = useState<GridState>({
        '0-0': 'A',
        '0-1': 'B',
        '0-2': null,
        '1-0': null,
        '1-1': 'C',
        '1-2': null,
        '2-0': null,
        '2-1': null,
        '2-2': 'D',
    });

    const DraggableItem: React.FC<DraggableItemProps> = ({ id }) => {
        const { attributes, listeners, setNodeRef, transform } = useDraggable({
            id,
        });

        const style: React.CSSProperties = {
            transform: `translate3d(${transform?.x ?? 0}px, ${transform?.y ?? 0}px, 0)`,
            padding: '10px',
            background: 'lightblue',
            borderRadius: '4px',
            cursor: 'grab',
            textAlign: 'center',
            display: 'inline-block',
        };

        return (
            <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
                {id}
            </div>
        );
    };

    const DroppableCell: React.FC<DroppableCellProps> = ({ id, item }) => {
        const { isOver, setNodeRef } = useDroppable({
            id,
        });

        const style: React.CSSProperties = {
            width: '100px',
            height: '100px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            background: isOver ? 'lightgreen' : 'lightgray',
            border: '1px solid black',
            position: 'relative',
        };

        return (
            <div ref={setNodeRef} style={style}>
                {item && <DraggableItem id={item} />}
            </div>
        );
    };

    const handleDragEnd = (event: DragEndEvent): void => {
        const { active, over } = event;

        if (over && active.id !== over.id) {
            setGrid((prevGrid) => {
                const updatedGrid = { ...prevGrid };

                // Remove the dragged item from its original cell
                const sourceCell = Object.keys(prevGrid).find((key) => prevGrid[key] === active.id);
                if (sourceCell) updatedGrid[sourceCell] = null;

                // Place the dragged item in the target cell
                // @ts-ignore
                updatedGrid[over.id] = active.id;

                return updatedGrid;
            });
        }
    };

    return (
        <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '5px' }}>
                {initialGrid.flat().map((cellId) => (
                    <DroppableCell key={cellId} id={cellId} item={grid[cellId]} />
                ))}
            </div>
        </DndContext>
    );
}