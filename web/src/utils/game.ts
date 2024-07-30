import { Submission } from "@/types/submission";
import { Team } from "@/types/team";

export interface Row {
    id?: number;
    team: Team;
    submissions: Array<Submission>;
    rank?: number;
    totalScore: number;
    solvedCount: number;
}

export function calculateAndSort(
    submissions?: Array<Submission>
): Array<Row> | undefined {
    if (!submissions) return;

    // 初始化团队提交数据的对象
    const teamSubmissions: Record<number, Row> = {};

    submissions?.forEach((submission) => {
        const { team_id, team, pts } = submission;
        if (!teamSubmissions[Number(team_id)]) {
            teamSubmissions[Number(team_id)] = {
                team: team!,
                submissions: [],
                totalScore: 0,
                solvedCount: 0,
            };
        }
        teamSubmissions[Number(team_id)].submissions.push(submission);
        teamSubmissions[Number(team_id)].totalScore += pts || 0;
        teamSubmissions[Number(team_id)].solvedCount += pts || 0 > 0 ? 1 : 0;
    });

    // 将对象转换为数组并按总分降序排序
    const rowsArray = Object.values(teamSubmissions).sort(
        (a, b) => b.totalScore - a.totalScore
    );

    // 设置排名
    rowsArray.forEach((row, index) => {
        row.rank = index + 1;
    });
    return rowsArray;
}
