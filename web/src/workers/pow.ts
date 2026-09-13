import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";

self.onmessage = async (e) => {
  const { c, d } = e.data;
  let nonce = 0;
  let result = "";

  while (!result.startsWith("0".repeat(d + 1))) {
    nonce++;
    result = bytesToHex(sha256(c + nonce.toString(16)));
  }

  postMessage(c + nonce.toString(16));
};
