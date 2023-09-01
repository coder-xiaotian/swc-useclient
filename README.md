# use-client

[中文](https://github.com/coder-xiaotian/swc-useclient/blob/main/README-ZH.md)

[![NPM](https://img.shields.io/npm/l/use-client)](https://github.com/coder-xiaotian/swc-useclient/blob/main/LICENSE)
[![npm](https://img.shields.io/npm/v/use-client)](https://www.npmjs.com/package/use-client)
[![CI](https://github.com/coder-xiaotian/swc-useclient/actions/workflows/rust.yml/badge.svg)](https://github.com/coder-xiaotian/swc-useclient/actions/workflows/rust.yml)

A swc plugin that automatically converts React component libraries into "React Client Component". For example, you can automatically convert components from @mui into "React Client Component" without having to wrap a component that uses "use client".

## Installation

npm:

```
npm install -D use-client
```

yarn:

```
yarn add -D use-client
```

pnpm:

```
pnpm i -D use-client
```

## Configuration

| Property  | Type  | Description |
| ---- | ---- | ---- |
| include | (string \| IncludeConfig)[] ｜ Array of paths to be transformed |

IncludeConfig:
| Property | Type | Description |
| ---- | ---- | ---- | 
| path | string ｜ Path to match |
| insert | string | Custom content to insert at the beginning of the file, default value: "use client" |

## Examples
next.js configuration:

```js
const nextConfig = {
  experimental: {
    swcPlugins: [
      [
        "use-client",
        {
          include: ["@mui/material", "@mui/icons-material"，"antd", "@ant-design/icons"],
        },
      ],
    ],
  },
};
```

swc configuration:

```js
{
  "$schema": "https://json.schemastore.org/swcrc",
  "jsc": {
    "experimental": {
      "plugins": [
        ["use-client", {
          "include": ["@mui/material", "@mui/icons-material"，"antd", "@ant-design/icons"]
        }]
      ]
    }
  }
}
```

Custom Insert Content:
```js
const nextConfig = {
  experimental: {
    swcPlugins: [
      [
        'use-client',
        {
          include: ["path/to", {
            path: "path/to",
            insert: "use strict"
          }]
        }
      ],
    ]
  }
}
```

## FAQ

+ The swc plugin interrupts tree shaking

At present, this is an expected behavior, [The Next.js team will improve it in the future](https://github.com/vercel/next.js/issues/52679#issuecomment-1636807256)。You can temporarily solve this problem by configuring modularizeImports:
```javascript
const nextConfig = {
  modularizeImports: {
    "@mui/material": {
      transform: "@mui/material/{{member}}"
    },
  },
  experimental: {
    swcPlugins: [
      [
        "use-client",
        {
          include: ["@mui/material"],
        },
      ],
    ],
  },
};
```
Note that some functions do not follow the '@ mui/material/{{member}}' rule, such as createTheme and ThemeProvider, which need to be processed separately as' import {createTheme, ThemeProvider} from '@ mui/material/styles'`

🌟🌟🌟🌟 If this plugin is helpful to you, give the author a star.🙏🙏🙏🌟🌟🌟🌟
