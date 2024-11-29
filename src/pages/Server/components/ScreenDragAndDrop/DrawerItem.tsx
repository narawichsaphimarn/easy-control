import {
  Box,
  Card,
  CardContent,
  Divider,
  Drawer,
  List,
  Skeleton,
  Typography,
} from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { useCallback, useState } from "react";
import WifiFindIcon from "@mui/icons-material/WifiFind";
import { invoke } from "@tauri-apps/api/core";
import { ScreenMatrixRequest, ScreenScale } from "../..";

interface InterfaceProps {
  isOpen: number | null;
  setIsOpen: React.Dispatch<React.SetStateAction<number | null>>;
  addScreenMatrix: (data: ScreenMatrixRequest) => void;
}

export interface System {
  host_name: string;
  ip: string;
  mac: string;
  screen: ScreenScale;
}

export const DrawerItem = ({
  isOpen,
  setIsOpen,
  addScreenMatrix,
}: InterfaceProps) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [system, setSystem] = useState<System[]>([]);
  const [_, setSystemBak] = useState<System[]>([]);

  const scanNetwork = useCallback(async () => {
    const resultSystem = await invoke<System[]>("scan_machine");
    setSystem(resultSystem);
  }, [system]);

  const clickScan = async () => {
    setLoading(true);
    await scanNetwork()
      .then(() => {
        setLoading(false);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  const convertScreenMatrixAndAdd = (data: System) => {
    if (isOpen) {
      const screenMatrix: ScreenMatrixRequest = {
        screen_no: isOpen,
        machine: {
          host_name: data.host_name,
          ip: data.ip,
          mac: data.mac,
          screen: data.screen,
        },
      };
      setSystemBak((prev) => [...prev, data]);
      setSystem((prev) => {
        let prevSystem = [...prev];
        prevSystem = prevSystem.filter((item) => item.mac !== data.mac);
        return prevSystem;
      });
      addScreenMatrix(screenMatrix);
    }
    setIsOpen(null);
  };

  const DrawerList = (
    <Box
      sx={{ width: 300, display: "flex", flexDirection: "column" }}
      role="presentation"
    >
      <Box>
        <LoadingButton
          loading={loading}
          loadingPosition="start"
          onClick={clickScan}
          startIcon={<WifiFindIcon />}
          style={{ float: "right" }}
        >
          Scan machine
        </LoadingButton>
      </Box>
      <Box
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <List>
          {loading ? (
            <>
              <Box sx={{ pt: 0.5 }}>
                <Skeleton variant="rectangular" width={210} height={118} />
                <Skeleton />
                <Skeleton width="60%" />
              </Box>
              <Box sx={{ pt: 0.5 }}>
                <Skeleton variant="rectangular" width={210} height={118} />
                <Skeleton />
                <Skeleton width="60%" />
              </Box>
            </>
          ) : (
            <Box>
              {system.map((data) => (
                <Card
                  sx={{ minWidth: 275, cursor: "pointer" }}
                  onClick={() => {
                    convertScreenMatrixAndAdd(data);
                  }}
                >
                  <CardContent>
                    <Typography variant="h5" component="div">
                      {data.host_name}
                    </Typography>
                    <Typography
                      gutterBottom
                      sx={{
                        color: "text.secondary",
                        fontSize: 14,
                      }}
                    >
                      {data.ip}
                    </Typography>

                    <Typography
                      gutterBottom
                      sx={{
                        color: "text.secondary",
                        fontSize: 10,
                        float: "left",
                      }}
                    >
                      {data.mac}
                    </Typography>
                    <Divider orientation="vertical" variant="middle" flexItem />
                    <Typography
                      gutterBottom
                      sx={{
                        color: "text.secondary",
                        fontSize: 10,
                        float: "right",
                      }}
                    >
                      {`${data.screen.width} x ${data.screen.height}`}
                    </Typography>
                  </CardContent>
                </Card>
              ))}
            </Box>
          )}
        </List>
      </Box>
    </Box>
  );

  return (
    <Box style={{}}>
      <Drawer
        open={isOpen === null ? false : true}
        onClose={() => setIsOpen(null)}
        anchor={"right"}
      >
        {DrawerList}
      </Drawer>
    </Box>
  );
};
