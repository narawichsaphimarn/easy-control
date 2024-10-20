import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { Board, BoardScaleBody } from "../../components/board";
import { ShapeRect } from "../../components/shape/rect";
import "./stype.css";

interface ScreenBody {
  name: string;
  width: number;
  height: number;
  fill: string;
  isDragging: boolean;
  mac: string;
  x: number; // ตำแหน่ง x
  y: number; // ตำแหน่ง y
  role: string;
}

export const Home = () => {
  const homeRef = useRef(null);
  const [screen, setScreen] = useState<ScreenBody[]>([]);
  const [boardSize, setBoardSize] = useState<BoardScaleBody>({
    width: 0,
    height: 0,
  });

  useEffect(() => {
    getScreen();
  }, []);

  // useEffect(() => {
  //   handleObject();
  // }, [screen]);

  const getScreen = async () => {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const screen: ScreenBody[] = await invoke("get_screen", {});
    let newScreen: ScreenBody[] = await handleObject(screen);
    setScreen(newScreen);
  };

  const handleObject = async (screen: ScreenBody[]) => {
    return screen.map((i) => {
      console.log("i", i);
      if (i.role === "main") {
        const halfObjectX = (i.width * 0.08) / 2;
        const halfObjectY = (i.height * 0.08) / 2;
        const halfBoardX = boardSize.width / 2;
        const halfBoardY = boardSize.height / 2;
        i.x = halfBoardX - halfObjectX;
        i.y = halfBoardY - halfObjectY;
        return i;
      } else {
        return i;
      }
    });
  };

  const handleDragStart = (e: { target: { id: () => any } }) => {
    const id = e.target.id();
    setScreen(
      screen.map((screen) => {
        return {
          ...screen,
          isDragging: screen.mac === id,
        };
      })
    );
  };

  const handleDragEnd = () => {
    setScreen(
      screen.map((screen) => {
        return {
          ...screen,
          isDragging: false,
        };
      })
    );
  };

  // const handleDragEnd = (e: { target: any }) => {
  //   if (homeRef.current) {
  //     const draggedShape = e.target; // เข้าถึงวัตถุที่ถูกลาก
  //     const stage = e.target.getStage();
  //     const boardWidth = stage.width();
  //     const boardHeight = stage.height();
  //     const { x, y } = draggedShape.position(); // ตำแหน่งปัจจุบันของวัตถุ

  //     const rectWidth = draggedShape.width(); // กว้างของวัตถุ
  //     const rectHeight = draggedShape.height(); // สูงของวัตถุ

  //     // คำนวณตำแหน่งใหม่ที่ไม่ซ้อนกัน
  //     const newX = Math.max(0, Math.min(x, boardWidth - rectWidth));
  //     const newY = Math.max(0, Math.min(y, boardHeight - rectHeight));

  //     // ตรวจสอบการชนกันกับวัตถุอื่น ๆ
  //     const isOverlapping = screen.some((item) => {
  //       const itemX = item.x; // ต้องมีใน ScreenBody
  //       const itemY = item.y; // ต้องมีใน ScreenBody
  //       const itemWidth = item.width * 0.08; // คำนวณจากขนาด

  //       // ตรวจสอบว่ามีการชนกันหรือไม่
  //       return (
  //         newX < itemX + itemWidth &&
  //         newX + rectWidth > itemX &&
  //         newY < itemY + rectHeight &&
  //         newY + rectHeight > itemY
  //       );
  //     });

  //     // ถ้ามีการชน ให้ปรับตำแหน่ง
  //     let finalX = newX;
  //     let finalY = newY;

  //     if (isOverlapping) {
  //       // ตัวอย่างการขยับให้ชิดกัน
  //       const offset = rectWidth + 1; // ระยะห่างที่ต้องการให้วัตถุไม่ซ้อนกัน
  //       finalX = newX + offset; // ขยับให้ชิดกัน
  //     }

  //     // อัปเดตตำแหน่งของวัตถุใน state
  //     setScreen(
  //       screen.map((item) => {
  //         return item.mac === draggedShape.id()
  //           ? { ...item, x: finalX, y: finalY }
  //           : item;
  //       })
  //     );
  //   }
  // };

  const handleDragMove = (e: any) => {
    if (homeRef.current) {
      const stage = e.target.getStage();
      const { x, y } = e.target.position();
      const rectWidth = e.target.width();
      const rectHeight = e.target.height();
      const boardWidth = stage.width();
      const boardHeight = stage.height();
      // ตรวจสอบและปรับขอบเขตให้ Rect ไม่เกินขอบของ Board
      const newX = Math.max(0, Math.min(x, boardWidth - rectWidth - 16));
      const newY = Math.max(0, Math.min(y, boardHeight - rectHeight));

      const otherTarget = screen.filter((item) => item.mac !== e.target.id());

      otherTarget.forEach((i) =>
        console.log(`${i.fill} - ${Number(newX) - Number(i.x)}`)
      );
      // const objectEdjoinX = otherTarget.reduce((min, current) => ,
      // newX);
      console.log("otherTarget", otherTarget);
      e.target.position({ x: newX, y: newY });
    }
  };

  return (
    <div ref={homeRef} className="home_container">
      <Board
        className="home_board_container"
        width={window.innerWidth}
        height={window.innerHeight * 0.6}
        getScale={setBoardSize}
      >
        {screen.map((item, index) => {
          return (
            <ShapeRect
              id={item.mac}
              key={index}
              x={item.x}
              y={item.y}
              width={item.width * 0.08}
              height={item.height * 0.08}
              fill={item.fill}
              draggable
              onDragStart={handleDragStart}
              onDragEnd={handleDragEnd}
              stroke={item.isDragging ? "white" : ""}
              onDragMove={handleDragMove}
            />
          );
        })}
      </Board>
      <div className="home_log_container">
        <h1>Hello</h1>
      </div>
    </div>
  );
};
