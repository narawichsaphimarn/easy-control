import { useNavigate } from "react-router-dom";
import { Pages } from "../../constant";
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import IconButton from "@mui/material/IconButton";
import {Box} from "@mui/material";
import {TransferList} from "./components/TransferList";

export const Setting = () => {
  const navigate = useNavigate();
  return (
    <div style={{position: "relative"}}>
      <h1>Setting</h1>
        <IconButton
            aria-label="setting"
            style={{ color: "white", position: "absolute", top: 0, left: 0 }}
            onClick={() => navigate(Pages.HOME)}
        >
            <ArrowBackIcon />
        </IconButton>
        <Box>
            <TransferList />
        </Box>
    </div>
  );
};
