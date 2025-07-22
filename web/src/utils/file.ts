import type { Progress } from "ky";
import type { WebResponse } from "@/types";

export async function uploadFile(
  url: string,
  file: File[],
  onUploadProgress?: (progress: Progress) => void
): Promise<WebResponse<unknown>> {
  const formData = new FormData();
  for (const f of file) {
    formData.append(f.name, f);
  }
  return new Promise<WebResponse<unknown>>((resolve, reject) => {
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
      if (xhr.readyState === 4 && xhr.status === 200) {
        resolve(JSON.parse(xhr.responseText));
      } else {
        reject(JSON.parse(xhr.responseText));
      }
    };
    xhr.open("POST", url, true);
    xhr.send(formData);
  });
}
