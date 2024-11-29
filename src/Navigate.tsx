import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { Server } from "./pages/Server";
import { Client } from "./pages/Client";
import { Pages } from "./constant";

interface NavigateProps {
  role?: string;
}

export const Navigate = ({ role }: NavigateProps) => {
  return (
    <Router>
      <Routes>
        <Route
          path={Pages.HOME}
          element={
            role?.toLowerCase().endsWith("server") ? <Server /> : <Client />
          }
        />
      </Routes>
    </Router>
  );
};
