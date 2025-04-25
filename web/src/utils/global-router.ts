import { NavigateFunction } from "react-router";

const globalRouter = { navigate: undefined } as {
    navigate?: NavigateFunction;
};

export default globalRouter;
