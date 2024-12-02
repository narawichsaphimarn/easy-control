import { Box, Button, ButtonGroup, Typography } from "@mui/material";
import {
  DragChildRef,
  ScreenDragAndDrop,
} from "./components/ScreenDragAndDrop";
import { invoke } from "@tauri-apps/api/core";
import SendIcon from "@mui/icons-material/Send";
import SwitchRightIcon from "@mui/icons-material/SwitchRight";
import { useEffect, useRef, useState } from "react";
import LoadingButton from "@mui/lab/LoadingButton";
import RotateLeftIcon from "@mui/icons-material/RotateLeft";
import PowerSettingsNewIcon from "@mui/icons-material/PowerSettingsNew";

export interface ScreenMatrixRequest {
  screen_no: number;
  machine: {
    host_name: string;
    ip: string;
    mac: string;
    screen: ScreenScale;
  };
}

export interface ScreenScale {
  width: number;
  height: number;
}

export interface Stores {
  screen_mapping_matrix: ScreenMapping[];
  screen_selector: ScreenSelector[];
}

export interface ScreenSelector {
  ip: string;
  mac: string;
  hostname: string;
  width: string;
  height: string;
  screen_no: number;
}

export interface ScreenMapping {
  mac_source: string;
  mac_target: string;
  edge: string;
}

export const Server = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [_, setScreenSelector] = useState<ScreenSelector[]>([]);
  const [screenMatrix, setScreenMatrix] = useState<ScreenMatrixRequest[]>([]);
  const [screenMapping, setScreenMapping] = useState<ScreenMapping[]>([]);
  const [screenMatrixCurrent, setScreenMatrixCurrent] = useState<string>("");
  const dragChildRef = useRef<DragChildRef>(null);
  const [isStartServer, setIsStartServer] = useState<boolean>(false);

  const getScreenSelector = async () => {
    await invoke<Stores>("get_screen_selector").then((result) => {
      setScreenSelector(result.screen_selector);
      setScreenMapping(result.screen_mapping_matrix);
      const mapScreen = mappingMatrix(result.screen_selector);
      setScreenMatrixCurrent(JSON.stringify(mapScreen));
      setScreenMatrix(mapScreen);
    });
  };

  useEffect(() => {
    getScreenSelector().catch((error) => console.error(error));
  }, []);

  const mappingMatrix = (data: ScreenSelector[]) => {
    return data
      .map((item) => {
        const map: ScreenMatrixRequest = {
          screen_no: item.screen_no,
          machine: {
            host_name: item.hostname,
            ip: item.ip,
            mac: item.mac,
            screen: {
              width: parseInt(item.width),
              height: parseInt(item.height),
            },
          },
        };
        return map;
      })
      .sort();
  };

  const switchRole = async () => {
    await invoke("switch_role");
  };

  const updateScreenMatrix = async () => {
    setLoading(true);
    await invoke("set_machine", { machine_select: screenMatrix })
      .then(() => {
        setLoading(false);
        getScreenSelector().catch((error) => console.error(error));
      })
      .catch((error) => console.error(error));
  };

  const resetMatrix = async () => {
    getScreenSelector()
      .then(() => {
        if (dragChildRef.current) {
          dragChildRef.current.reset();
        }
      })
      .catch((error) => console.error(error));
  };

  const startServer = async () => {
    await invoke("start_server")
      .then(() => {
        setIsStartServer(true);
      })
      .catch((error) => console.error(error));
  };

  const stopServer = async () => {
    await invoke("stop_server")
      .then(() => {
        setIsStartServer(false);
      })
      .catch((error) => console.error(error));
  };

  return (
    <div
      style={{
        position: "relative",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <Box>
        <Button
          color="secondary"
          onClick={switchRole}
          endIcon={<SwitchRightIcon />}
        >
          Switch to Client
        </Button>
      </Box>
      <Typography variant="h4" gutterBottom>
        Server
      </Typography>
      <Box
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          width: "60%",
          position: "relative",
        }}
      >
        <ScreenDragAndDrop
          ref={dragChildRef}
          screens={screenMatrix}
          setScreenMatrix={setScreenMatrix}
        />
        <div style={{ marginTop: "10px" }}>
          <ButtonGroup variant="outlined" aria-label="Loading button group">
            {isStartServer ? (
              <Button
                startIcon={<PowerSettingsNewIcon />}
                disabled={screenMapping.length === 0}
                onClick={stopServer}
              >
                Stop Server
              </Button>
            ) : (
              <Button
                startIcon={<PowerSettingsNewIcon />}
                disabled={screenMapping.length === 0}
                onClick={startServer}
              >
                Start Server
              </Button>
            )}
            <Button
              startIcon={<RotateLeftIcon />}
              disabled={JSON.stringify(screenMatrix) === screenMatrixCurrent}
              onClick={resetMatrix}
            >
              Reset
            </Button>
            <LoadingButton
              loading={loading}
              color="success"
              loadingPosition="start"
              startIcon={<SendIcon />}
              onClick={() => updateScreenMatrix()}
              disabled={JSON.stringify(screenMatrix) === screenMatrixCurrent}
            >
              Update
            </LoadingButton>
          </ButtonGroup>
        </div>
      </Box>
    </div>
  );
};
