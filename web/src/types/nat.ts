import { Pod } from "./pod";

export interface Nat {
	id: number;
	pod_id: number;
	pod: Pod;
	src_port: number;
	dst_port: number;
	proxy: string;
	entry: string;
}
