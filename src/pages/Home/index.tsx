import { useNavigate } from "react-router-dom";
import { ButtonIcon } from "../../components/button/icon";
import { Pages } from "../../constant";

export const Home = () => {
  const navigate = useNavigate();
  return (
    <div className="container mx-auto">
      <h1>Home</h1>
      <ButtonIcon label="Setting" onClick={() => navigate(Pages.SETTING)} icon={<i className="fa fa-cog text-base"></i>} />
    </div>
  );
};
