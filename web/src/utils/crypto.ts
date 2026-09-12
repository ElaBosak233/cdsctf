const textEncoder = new TextEncoder();

/** Encode a Unicode string as standard Base64. */
export function encodeBase64(value: string): string {
  const bytes = textEncoder.encode(value);
  let binary = "";

  for (const byte of bytes) {
    binary += String.fromCharCode(byte);
  }

  return btoa(binary);
}
