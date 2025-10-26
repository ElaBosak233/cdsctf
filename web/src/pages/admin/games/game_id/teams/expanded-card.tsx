import { useQuery } from "@tanstack/react-query";
import { LoaderCircleIcon } from "lucide-react";
import { getTeamUser } from "@/api/admin/games/game_id/teams/team_id/users";
import { Avatar } from "@/components/ui/avatar";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { Team } from "@/models/team";
import { cn } from "@/utils";

export interface ExpandedCardProps {
  team: Team;
}

function ExpandedCard(props: ExpandedCardProps) {
  const { team } = props;

  const { data: members, isLoading } = useQuery({
    queryKey: ["members", team.id, team.game_id],
    queryFn: () =>
      getTeamUser({
        team_id: team.id,
        game_id: team.game_id,
      }),
    select: (response) => response.data,
  });

  return (
    <div className="p-4 border-t text-sm text-muted-foreground flex justify-between items-center">
      <p className={cn(["text-secondary-foreground"])}>
        {team.slogan || "这个小队很懒，什么也没留下。"}
      </p>
      <div className="*:data-[slot=avatar]:ring-background flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:grayscale">
        {members?.map((member) => (
          <Tooltip key={member.id}>
            <TooltipTrigger>
              <Avatar
                src={member.has_avatar && `/api/users/${member.id}/avatar`}
                fallback={member.name?.charAt(0)}
              />
            </TooltipTrigger>
            <TooltipContent>
              <span>{member.name}</span>
            </TooltipContent>
          </Tooltip>
        ))}
        {isLoading && <LoaderCircleIcon className="animate-spin w-4 h-4" />}
      </div>
    </div>
  );
}

export { ExpandedCard };
