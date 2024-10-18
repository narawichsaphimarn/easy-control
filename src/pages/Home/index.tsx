import { Board } from "../../components/board";
import { ShapeRect } from "../../components/shape/rect";
import "./stype.css";
import { useRef, useEffect } from "react";
import Konva from "konva";

export const Home = () => {
  const homeRef = useRef(null);
  // useEffect(() => {
  //   if (boardRef.current) {
  //     const width = boardRef.current.width;
  //     const height = boardRef.current.height;
  //     console.log("Board Width:", width);
  //     console.log("Board Height:", height);
  //   }
  // }, []);

  return (
    <div ref={homeRef} className="home_container">
      <Board
        className="home_board_container"
        width={window.innerWidth}
        height={window.innerHeight * 0.6}
        getScale={(e: any) => {
          console.log("ietm", e);
        }}
      >
        {[...Array(3)].map((_, index) => {
          return (
            <ShapeRect
              key={index}
              x={0}
              y={0}
              width={60}
              height={60}
              fill={"red"}
              draggable
            />
          );
        })}
      </Board>
      <div className="home_log_container">
        {/* <ButtonIcon
          label="Setting"
          onClick={() => navigate(Pages.SETTING)}
          icon={<i className="fa fa-cog text-base"></i>}
        /> */}
        <h1>Hello</h1>
      </div>
    </div>
  );
};
