import { Box, Button, ButtonGroup, Typography } from "@mui/material";
import { ScreenDragAndDrop } from "./components/ScreenDragAndDrop";
import { invoke } from "@tauri-apps/api/core";
import SendIcon from "@mui/icons-material/Send";
import SwitchRightIcon from "@mui/icons-material/SwitchRight";
import { useEffect, useState } from "react";
import LoadingButton from "@mui/lab/LoadingButton";
import RotateLeftIcon from "@mui/icons-material/RotateLeft";

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

export interface ScreenSelector {
  ip: string;
  mac: string;
  hostname: string;
  width: string;
  height: string;
  screen_no: number;
}

export const Server = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [_, setScreenSelector] = useState<ScreenSelector[]>([]);
  const [screenMatrix, setScreenMatrix] = useState<ScreenMatrixRequest[]>([]);
  const [screenMatrixCurrent, setScreenMatrixCurrent] = useState<string>("");

  const getScreenSelector = async () => {
    await invoke<ScreenSelector[]>("get_screen_selector").then((result) => {
      setScreenSelector(result);
      const mapScreen = mappingMatrix(result);
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
    getScreenSelector().catch((error) => console.error(error));
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
        }}
      >
        <ScreenDragAndDrop
          screens={screenMatrix}
          setScreenMatrix={setScreenMatrix}
        />
        <div style={{ marginTop: "10px" }}>
          <ButtonGroup variant="outlined" aria-label="Loading button group">
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
