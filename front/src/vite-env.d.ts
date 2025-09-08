/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly DEV: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
