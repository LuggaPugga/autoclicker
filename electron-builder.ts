import type { Configuration } from 'electron-builder'

export default {
  appId: 'dev.luggapugga.autoclicker',
  productName: 'autoclicker',

  directories: {
    buildResources: 'build',
    output: `dist/`,
  },

  files: [
    '!**/.vscode/*',
    '!src/*',
    '!electron.vite.config.{js,ts,mjs,cjs}',
    '!{.eslintignore,.eslintrc.cjs,.prettierignore,.prettierrc.yaml,dev-app-update.yml,CHANGELOG.md,README.md}',
    '!{.env,.env.*,.npmrc,pnpm-lock.yaml}',
    '!{tsconfig.json}',
  ],

  publish: {
    provider: 'github',
    owner: 'LuggaPugga',
    repo: 'autoclicker',
    token: process.env.GITHUB_TOKEN,
    releaseType: 'prerelease',
  },

  asarUnpack: ['resources/**'],

  mac: {
    entitlementsInherit: 'build/entitlements.mac.plist',
    extendInfo: [
      { NSCameraUsageDescription: "Application requests access to the device's camera." },
      { NSMicrophoneUsageDescription: "Application requests access to the device's microphone." },
      {
        NSDocumentsFolderUsageDescription:
          "Application requests access to the user's Documents folder.",
      },
      {
        NSDownloadsFolderUsageDescription:
          "Application requests access to the user's Downloads folder.",
      },
    ],
    notarize: false,
    target: ['zip', 'dmg', 'dir'],
    artifactName: '${name}-${version}.${ext}',
  },

  dmg: {
    artifactName: '${name}-${version}.${ext}',
  },

  linux: {
    target: ['AppImage', 'snap', 'deb'],
    maintainer: 'LuggaPugga',
    category: 'Utility',
    artifactName: '${name}-${version}.${ext}',
  },

  appImage: {
    artifactName: '${name}-${version}.${ext}',
  },

  win: {
    executableName: 'autoclicker',
    target: ['portable'],
    artifactName: '${name}-${version}.${ext}',
  },

  nsis: {
    artifactName: '${name}-${version}.${ext}',
    shortcutName: '${productName}',
    uninstallDisplayName: '${productName}',
    createDesktopShortcut: 'always',
  },
} satisfies Configuration
