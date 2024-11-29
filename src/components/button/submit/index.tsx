import React from "react";
import { ButtonProps } from "../struct";

export const ButtonSubmit: React.FC<ButtonProps> = (props: ButtonProps) => {
  return (
    <button onClick={props.onClick} disabled={props.disabled}>
      {props.label}
    </button>
  );
};
