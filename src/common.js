import { platform } from '@tauri-apps/plugin-os';
import { readFile } from '@tauri-apps/plugin-fs';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

export const addPublication = async () => {
    const file = await open({
        multiple: false,
        filters: [{
            name: "JWPUB file",
            extensions: ["jwpub"],
        }],
        directory: false,
    })

    if (platform() == "android") {
        const contents = await readFile(file);
        await invoke("catalog_install_jwpub_from_archive", {file: contents})

    } else {
        await invoke("catalog_install_jwpub_file", {filePath: file})
    }   
}