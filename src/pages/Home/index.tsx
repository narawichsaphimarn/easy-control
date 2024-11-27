import "./stype.css";
// import {ButtonSubmit} from "../../components/button/submit";
// import {invoke} from "@tauri-apps/api/core";

export const Home = () => {
    // const [role, setRole] = useState<string>("");
  // const getScreen = async () => {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   const screens = await invoke("get_machine", {});
  //   // let newScreen: ScreenBody[] = await handleObject(screen);
  //   console.log(screens);
  // }

    // const restartApp = () => {
    //     invoke("restart_app").catch((error) => {
    //         console.error("Failed to restart the application:", error);
    //     });
    // }

    // useEffect(() => {
    //     getRole().catch((error) => {
    //         console.error("Failed to get role:", error);
    //     });
    // }, [])

  return (
      <div className="container mx-auto">
          <h1>Home</h1>
          {/*<ButtonSubmit label="Loading Screen" onClick={() => {restartApp()}} />*/}
      </div>
  );
};
