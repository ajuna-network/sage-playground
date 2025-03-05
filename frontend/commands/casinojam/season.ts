import { Command } from "@/types/command";
import { isCasinoJamApi } from "./util";

export const season: Command = {
  execute: async (_, { api }) => {
    if (!api || !isCasinoJamApi(api)) return "No CasinoJam API available";

    const seasonStatus =
      await api.query.CasinoJamSeasons.CurrentSeasonStatus.getValue();

    const seasonSchedules =
      await api.query.CasinoJamSeasons.SeasonSchedules.getValue(
        seasonStatus?.season_id ?? 0
      );

    return `Season: ${JSON.stringify(
      { seasonStatus, seasonSchedules },
      null,
      2
    )}`;
  },
  help: {
    command: "season",
    description: "Display the season info",
  },
};
