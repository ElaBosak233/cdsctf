import { useAuth } from "@/utils/axios";

export function useConfigApi() {
	const auth = useAuth();

	const getPltCfg = () => {
		return auth.get("/configs");
	};

	return {
		getPltCfg,
	};
}
