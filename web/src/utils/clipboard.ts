async function copyToClipboard(text: string): Promise<boolean> {
    try {
        if (navigator.clipboard) {
            await navigator.clipboard.writeText(text);
            return true;
        } else {
            const textarea = document.createElement("textarea");
            textarea.value = text;
            document.body.appendChild(textarea);
            textarea.select();
            const success = document.execCommand("copy");
            document.body.removeChild(textarea);
            return success;
        }
    } catch (error) {
        console.error(error);
        return false;
    }
}

export { copyToClipboard };
