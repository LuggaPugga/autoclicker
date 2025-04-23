import { defineConfig } from 'eslint/config'

export default defineConfig([
  {
    extends: [
      'eslint:recommended',
      'plugin:react/recommended',
      'plugin:react/jsx-runtime',
      '@electron-toolkit/eslint-config-ts/recommended',
      '@electron-toolkit/eslint-config-prettier',
    ],
  },
])
