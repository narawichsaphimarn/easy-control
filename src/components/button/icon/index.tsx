import React, { ReactNode } from "react";
import { ButtonProps } from "@components/button/struct";

interface ButtonIconProps extends ButtonProps {
  icon: ReactNode;
}

export const ButtonIcon: React.FC<ButtonIconProps> = (
  props: ButtonIconProps
) => {
  return (
    <button onClick={props.onClick} disabled={props.disabled}>
      {props.icon}
    </button>
  );
};
