# use-client

[中文](https://github.com/coder-xiaotian/swc-useclient/blob/main/README-ZH.md)

[![NPM](https://img.shields.io/npm/l/use-client)](https://github.com/coder-xiaotian/swc-useclient/blob/main/LICENSE)
[![npm](https://img.shields.io/npm/v/use-client)](https://www.npmjs.com/package/use-client)
[![Rust](https://github.com/coder-xiaotian/swc-useclient/actions/workflows/rust.yml/badge.svg)](https://github.com/coder-xiaotian/swc-useclient/actions/workflows/rust.yml)

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

+ include：["ui-library-path"]

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
