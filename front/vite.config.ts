import { viteStaticCopy } from "vite-plugin-static-copy";

export default {
    plugins: [
        viteStaticCopy({
            targets: [
                {
                    src: "node_modules/cm-chessboard/assets",
                    dest: "."
                }
            ]
        })
    ]
};
