import { invoke } from "@tauri-apps/api/core";
import { useMemo, useState } from "react";
import { Box, Button, Typography } from "@mui/material";
import SwitchLeftIcon from '@mui/icons-material/SwitchLeft';

interface System {
  host_name: string;
  ip: string;
  mac: string;
  screen: {
    width: number;
    height: number;
  };
}

export const Client = () => {
  const [system, setSystem] = useState<System>();
  useMemo(async () => await invoke<System>("get_system_detail"), []).then((r) =>
    setSystem(r)
  );

  const switchRole = async () => {
    await invoke("switch_row");
  };

  return (
    <div>
      <Box>
        <Button color="secondary" onClick={switchRole} endIcon={<SwitchLeftIcon />}>
          Switch to Server
        </Button>
      </Box>
      <Typography variant="h4" gutterBottom>
        Client
      </Typography>
      <Box>
        <Typography variant="h1" gutterBottom>
          {system?.ip}
        </Typography>
        <Typography variant="h2" gutterBottom>
          {system?.host_name}
        </Typography>
        <Typography variant="h5" gutterBottom>
          {`${system?.screen.width} x ${system?.screen.height}`}
        </Typography>
        <Typography variant="h6" gutterBottom>
          {system?.mac}
        </Typography>
      </Box>
    </div>
  );
};
