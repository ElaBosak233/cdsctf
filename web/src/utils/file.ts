import type { Progress } from "ky";

function parseXHRResponse(xhr: XMLHttpRequest): unknown {
  const text = xhr.responseText;
  if (!text) return undefined;

  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}

export async function uploadFile<T = unknown>(
  url: string,
  file: File[],
  onUploadProgress?: (progress: Progress) => void
): Promise<T> {
  const formData = new FormData();
  for (const f of file) {
    formData.append(f.name, f);
  }
  return new Promise<T>((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable) {
        onUploadProgress?.({
          percent: (e.loaded / e.total) * 100,
          transferredBytes: e.loaded,
          totalBytes: e.total,
        });
      }
    };
    xhr.onloadend = () => {
      const payload = parseXHRResponse(xhr);
      if (xhr.readyState === 4 && xhr.status >= 200 && xhr.status < 300) {
        resolve(payload as T);
      } else {
        reject(payload);
      }
    };
    xhr.open("POST", url, true);
    xhr.send(formData);
  });
}
