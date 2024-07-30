import { Game } from "./game";
import { Team } from "./team";

export interface GameTeam {
    id?: number;
    team_id?: number;
    team?: Team;
    game_id?: number;
    game?: Game;
    rank?: number;
    pts?: number;
    solved?: number;
    is_allowed?: boolean;
    signature?: string;
}
