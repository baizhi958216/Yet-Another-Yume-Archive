#!/usr/bin/env node
// Regenerates providers/bundle from a local (gitignored) bundle.config.json.
//
//   bundle.config.json:
//   {
//     "providers": [
//       { "crate": "yaya-extra-provider-xxx",
//         "path": "/abs/path/to/provider-repo/shim/yaya-extra-provider-xxx" }
//       // or: { "crate": "...", "git": "https://…", "tag": "v0.1.0" }
//     ]
//   }
//
// Without a config file the checked-in defaults (direct provider only) are
// restored. The host repo's git history never names a site provider — the
// generated files are only ever produced locally or in CI.

import { copyFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const bundleDir = join(root, 'providers', 'bundle')
const configPath = join(root, 'bundle.config.json')

if (!existsSync(configPath)) {
  copyFileSync(join(bundleDir, 'Cargo.default.toml'), join(bundleDir, 'Cargo.toml'))
  copyFileSync(join(bundleDir, 'src', 'lib.default.rs'), join(bundleDir, 'src', 'lib.rs'))
  console.log('gen-bundle: no bundle.config.json — restored defaults (direct only)')
  process.exit(0)
}

const config = JSON.parse(readFileSync(configPath, 'utf8'))
const providers = config.providers ?? []

const dependencyLines = providers.map((provider) => {
  if (provider.path)
    return `${provider.crate} = { path = ${JSON.stringify(provider.path)} }`
  if (provider.git) {
    const pin = provider.tag
      ? `, tag = ${JSON.stringify(provider.tag)}`
      : provider.rev
        ? `, rev = ${JSON.stringify(provider.rev)}`
        : ''
    return `${provider.crate} = { git = ${JSON.stringify(provider.git)}${pin} }`
  }
  throw new Error(`provider ${provider.crate}: needs "path" or "git"`)
})

const cargo = `${readFileSync(join(bundleDir, 'Cargo.default.toml'), 'utf8').trimEnd()}
${dependencyLines.join('\n')}
`

const pushes = providers
  .map(provider => `    values.push(${provider.crate.replaceAll('-', '_')}::provider());`)
  .join('\n')
const lib = `${readFileSync(join(bundleDir, 'src', 'lib.default.rs'), 'utf8')
  .trimEnd()
  .replace(/\n\s*values\n}$/m, `\n${pushes}\n    values\n}`)}
`

writeFileSync(join(bundleDir, 'Cargo.toml'), cargo)
writeFileSync(join(bundleDir, 'src', 'lib.rs'), lib)
console.log(`gen-bundle: bundled ${providers.length} extra provider(s): ${providers.map(p => p.crate).join(', ')}`)
