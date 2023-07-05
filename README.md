# use-client
[中文](https://github.com/coder-xiaotian/swc-useclient/blob/main/README-ZH.md)

A swc plugin that automatically converts component libraries into "React Client Component". For example, you can automatically convert components from @mui into "React Client Component" without having to wrap a component that uses "use client".

## Configuration
+ include: ["@mui/material"]

## Examples
next.js configuration:
```js
const nextConfig = {
  experimental: {
    swcPlugins: [
      [
        'use-client',
        {
          include: ["@mui/material"]
        }
      ],
    ]
  }
}
```

swc configuration:
```js
{
  "$schema": "https://json.schemastore.org/swcrc",
  "jsc": {
    "experimental": {
      "plugins": [
        ["use-client", {
          "include": ["@mui/material"]
        }]
      ]
    }
  }
}
```

