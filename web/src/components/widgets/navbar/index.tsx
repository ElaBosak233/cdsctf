import { useLocation, useParams } from "react-router";
import { Container } from "./container";
import { Context } from "./context";
import { useMemo } from "react";

function Navbar() {
    const location = useLocation();
    const pathname = location.pathname;
    const { game_id } = useParams<{ game_id?: string }>();

    const mode = useMemo(() => {
        if (pathname.startsWith("/games") && game_id) {
            return "game";
        }
        return "default";
    }, [pathname, game_id]);

    return (
        <Context.Provider value={{ mode }}>
            <Container />
        </Context.Provider>
    );
}

export { Navbar };
