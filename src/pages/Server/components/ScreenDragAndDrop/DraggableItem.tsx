import { useDraggable } from "@dnd-kit/core";
import { ScreenMatrixRequest } from "../..";

interface DraggableItemProps {
  id: number;
  item: ScreenMatrixRequest | null;
}

export const DraggableItem: React.FC<DraggableItemProps> = ({ id, item }) => {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id,
  });

  const style: React.CSSProperties = {
    transform: `translate3d(${transform?.x ?? 0}px, ${transform?.y ?? 0}px, 0)`,
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
      {item?.machine.host_name}
    </div>
  );
};
