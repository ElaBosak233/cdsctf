import { ConfigUpdateRequest } from "@/types/config";
import { useAuth } from "@/utils/axios";

export function useConfigApi() {
	const auth = useAuth();

	const getPltCfg = () => {
		return auth.get("/configs/");
	};

	const updatePltCfg = (request: ConfigUpdateRequest) => {
		return auth.put("/configs/", request);
	};

	return {
		getPltCfg,
		updatePltCfg,
	};
}
