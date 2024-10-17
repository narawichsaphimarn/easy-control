import React from "react";
import { useNavigate } from "react-router-dom";
import { ButtonSubmit } from "../../components/button/submit";
import { Pages } from "../../constant";

export const Setting = () => {
  const navigate = useNavigate();
  return (
    <div className="container mx-auto">
      <h1>Setting</h1>
      <ButtonSubmit label="Setting" onClick={() => navigate(Pages.HOME)} />
    </div>
  );
};
