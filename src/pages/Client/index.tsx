import {invoke} from "@tauri-apps/api/core";
import {useMemo, useState} from "react";
import {Box, Typography} from "@mui/material";

interface System {
    host_name: string,
    ip: string,
    mac: string,
    screen: {
        width: number,
        height: number,
    },
}



export const Client = () => {
    const [system, setSystem] = useState<System>();
    useMemo(async () => await invoke<System>("get_system_detail"), []).then(r => setSystem(r));

    return <div>
        <h1>Client</h1>
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
    </div>;
};