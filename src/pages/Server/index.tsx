import {Box, Drawer} from "@mui/material";
import React from "react";
import {ScreenDragAndDrop} from "./components/ScreenDragAndDrop";

export const Server = () => {
    const [open, setOpen] = React.useState(false);

    const toggleDrawer = (newOpen: boolean) => () => {
        setOpen(newOpen);
    };


    return (
    <div style={{position: "relative"}}>
      <h1>Server</h1>
        <Box style={{display: "flex", alignItems: "center", justifyContent: "center"}}>
            <Drawer open={open} onClose={toggleDrawer(false)} anchor={"right"}>
            </Drawer>
            <ScreenDragAndDrop />
        </Box>
    </div>
);
};
