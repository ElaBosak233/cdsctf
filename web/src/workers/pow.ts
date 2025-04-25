import CryptoJS from "crypto-js";

self.onmessage = async function (e) {
    const { c, d } = e.data;
    let nonce = 0;
    let result = "";

    while (!result.startsWith("0".repeat(d + 1))) {
        nonce++;
        result = CryptoJS.SHA256(c + nonce.toString(16)).toString();
    }

    postMessage(c + nonce.toString(16));
};
