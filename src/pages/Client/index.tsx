import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import { Box, Button, Card, CardContent, Typography } from "@mui/material";
import SwitchLeftIcon from "@mui/icons-material/SwitchLeft";

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
  const getSystemDetail = useCallback(async () => {
    const system = await invoke<System>("get_system_detail");
    setSystem(system);
  }, [system]);

  useEffect(() => {
    getSystemDetail();
  }, []);

  const switchRole = async () => {
    await invoke("switch_role");
  };

  return (
    <div>
      <Box>
        <Button
          color="secondary"
          onClick={switchRole}
          endIcon={<SwitchLeftIcon />}
        >
          Switch to Server
        </Button>
      </Box>
      <Typography variant="h4" gutterBottom>
        Client
      </Typography>
      <Box
        style={{
          width: "100%",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Card sx={{ maxWidth: 500, background: "#EBEAFF" }}>
          <CardContent>
            <Typography variant="h2" gutterBottom>
              {system?.ip}
            </Typography>
            <Typography
              variant="h3"
              gutterBottom
              sx={{ color: "text.secondary", mb: 1.5 }}
            >
              {system?.host_name}
            </Typography>
            <Typography
              variant="h5"
              gutterBottom
              sx={{ color: "text.secondary", mb: 1.5 }}
            >
              {`${system?.screen.width} x ${system?.screen.height}`}
            </Typography>
            <Typography
              variant="h6"
              gutterBottom
              sx={{ color: "text.secondary", mb: 1.5 }}
            >
              {system?.mac}
            </Typography>
          </CardContent>
        </Card>
      </Box>
    </div>
  );
};
