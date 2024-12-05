import {useDraggable} from "@dnd-kit/core";
import {ScreenMatrixRequest} from "../..";
import React from "react";

interface DraggableItemProps {
    id: number;
    item: ScreenMatrixRequest | null;
}

export const DraggableItem: React.FC<DraggableItemProps> = ({id, item}) => {
    const {attributes, listeners, setNodeRef, transform, over} = useDraggable({
        id,
    });

    const style: React.CSSProperties = {
        transform: `translate3d(${transform?.x ?? 0}px, ${transform?.y ?? 0}px, 0)`,
        background: "#4335A7",
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
        zIndex: over?.id === 999 ? 0 : 999,
        boxShadow: "2px 2px 3px #4335A7",
        // textOverflow: "ellipsis",
        overflow: "hidden",
        whiteSpace: "wrap"
    };

    return (
        <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
            <p>{item?.machine.host_name}</p>
        </div>
    );
};
