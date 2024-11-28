import { Box, Button, Typography } from "@mui/material";
import { ScreenDragAndDrop } from "./components/ScreenDragAndDrop";
import { invoke } from "@tauri-apps/api/core";
import SendIcon from "@mui/icons-material/Send";
import SwitchRightIcon from "@mui/icons-material/SwitchRight";
import { useMemo, useState } from "react";
import LoadingButton from "@mui/lab/LoadingButton";

interface ScreenMatrixRequest {
  screen_no: number;
  machine: {
    host_name: string;
    ip: string;
    mac: string;
    screen: {
      width: number;
      height: number;
    };
  };
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
  const [screenSelector, setScreenSelector] = useState<ScreenSelector[]>([]);
  const [screenMatrix, setScreenMatrix] = useState<ScreenMatrixRequest[]>([]);
  useMemo(
    async () => await invoke<ScreenSelector[]>("get_screen_selector"),
    []
  ).then((result) => setScreenSelector(result));

  const switchRole = async () => {
    await invoke("switch_row");
  };

  const updateScreenMatrix = async () => {
    setLoading(true);
    await invoke("set_machine", { machine_select: screenMatrix })
      .then((result) => {
        setLoading(false);
        console.log("response from mapping matrix {}", result);
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
        }}
      >
        <ScreenDragAndDrop screenSelector={screenSelector} />
        <div style={{ width: "100%", marginTop: "5px" }}>
          <LoadingButton
            loading={loading}
            color="success"
            loadingPosition="start"
            startIcon={<SendIcon />}
            variant="contained"
            onClick={() => updateScreenMatrix()}
            style={{ float: "right" }}
          >
            Update
          </LoadingButton>
        </div>
      </Box>
    </div>
  );
};
