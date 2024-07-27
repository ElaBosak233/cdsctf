import { Pod } from "./pod";

export interface Nat {
	pod: Pod;
	src: number;
	dst: number;
	proxy: string;
	entry: string;
}
