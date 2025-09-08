// @ts-check

import eslint from "@eslint/js";
import tseslint from "typescript-eslint";

export default tseslint.config(
    eslint.configs.recommended,
    tseslint.configs.recommended,
    {
        rules: {
            "no-implicit-coercion": "error",
            curly: "error",
            eqeqeq: "error",
            "@typescript-eslint/no-unused-vars": "warn"
        },
    },
);
