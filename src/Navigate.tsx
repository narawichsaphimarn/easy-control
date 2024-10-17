import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { Home } from "./pages/Home";
import { Setting } from "./pages/Setting";
import { Pages } from "./constant";

export const Navigate = () => {
  return (
    <Router>
      <Routes>
        <Route path={Pages.HOME} element={<Home />} />
        <Route path={Pages.SETTING} element={<Setting />} />
      </Routes>
    </Router>
  );
};
