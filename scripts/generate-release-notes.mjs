#!/usr/bin/env node

import { execFileSync } from 'node:child_process'
import process from 'node:process'

const tag = process.argv[2]

if (!tag || !/^v\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(tag)) {
  console.error('Usage: node scripts/generate-release-notes.mjs <version-tag>')
  process.exit(1)
}

function git(...args) {
  return execFileSync('git', args, { encoding: 'utf8' }).trim()
}

try {
  git('rev-parse', '--verify', `${tag}^{commit}`)
}
catch {
  console.error(`Release tag does not exist: ${tag}`)
  process.exit(1)
}

const previousTag = git('tag', '--merged', tag, '--list', 'v*', '--sort=-v:refname')
  .split('\n')
  .find(candidate => candidate && candidate !== tag)
const revision = previousTag ? `${previousTag}..${tag}` : tag
const repositoryUrl = process.env.GITHUB_REPOSITORY
  ? `${process.env.GITHUB_SERVER_URL ?? 'https://github.com'}/${process.env.GITHUB_REPOSITORY}`
  : git('remote', 'get-url', 'origin')
      .replace(/^git@github\.com:/, 'https://github.com/')
      .replace(/\.git$/, '')

const categoryOrder = [
  ['feat', 'New features'],
  ['fix', 'Bug fixes'],
  ['perf', 'Performance'],
  ['refactor', 'Refactoring'],
  ['docs', 'Documentation'],
  ['build', 'Build and packaging'],
  ['ci', 'Continuous integration'],
  ['other', 'Other changes'],
]
const categories = new Map(categoryOrder.map(([type]) => [type, []]))
const log = execFileSync(
  'git',
  ['log', '--format=%H%x1f%s%x00', revision],
  { encoding: 'utf8' },
)

for (const record of log.split('\0')) {
  const [hash, subject] = record.trim().split('\x1F')
  if (!hash || !subject || /^chore(?:\([^)]+\))?: release v/i.test(subject))
    continue

  const conventional = subject.match(/^([a-z]+)(?:\(([^)]+)\))?!?:\s+(\S.*)$/i)
  const type = conventional?.[1].toLowerCase() ?? 'other'
  const scope = conventional?.[2]
  const summary = conventional?.[3] ?? subject
  const category = categories.has(type) ? type : 'other'
  const scopePrefix = scope ? `**${scope}**: ` : ''
  categories.get(category).push(`- ${scopePrefix}${summary} ([${hash.slice(0, 7)}](${repositoryUrl}/commit/${hash}))`)
}

const sections = categoryOrder
  .filter(([type]) => categories.get(type).length > 0)
  .map(([type, heading]) => `## ${heading}\n\n${categories.get(type).join('\n')}`)

if (sections.length === 0)
  sections.push('No user-facing changes were recorded for this release.')

const changelogUrl = previousTag
  ? `${repositoryUrl}/compare/${previousTag}...${tag}`
  : `${repositoryUrl}/commits/${tag}`

process.stdout.write(`${sections.join('\n\n')}\n\n**Full changelog:** ${changelogUrl}\n`)
