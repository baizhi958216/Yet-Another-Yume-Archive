export type TaskStatus =
  | 'queued'
  | 'running'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'canceled'

export interface SelectOption {
  label: string
  value: unknown
  description?: string
}

export type FormField =
  | { key: string, label: string, description?: string, type: 'toggle', default: boolean }
  | { key: string, label: string, description?: string, type: 'select', options: SelectOption[], default: unknown }
  | { key: string, label: string, description?: string, type: 'text', default: string, placeholder?: string }
  | { key: string, label: string, description?: string, type: 'secret', default: string, placeholder?: string }
  | { key: string, label: string, description?: string, type: 'number', default: number, min?: number, max?: number, step?: number }

export interface TaskDraft {
  key: string
  title: string
  description: string
  size?: number
  imageUrl: string
  selected: boolean
  fields: FormField[]
  payload: unknown
}

export interface ProviderView {
  provider: string
  title: string
  description: string
  imageUrl: string
  tasks: TaskDraft[]
  fields: FormField[]
}

export interface CreateTasksRequest {
  provider: string
  source: string
  outputDir: string
  tasks: Array<{ draft: TaskDraft, options: Record<string, unknown> }>
  batchId?: string
  group?: string
}

export interface Artifact {
  path: string
  name: string
  mimeType: string
  size?: number
  metadata: Record<string, unknown>
}

export interface TaskSnapshot {
  id: string
  provider: string
  batchId?: string
  group?: string
  source: string
  draft: TaskDraft
  options: Record<string, unknown>
  outputDir: string
  status: TaskStatus
  completed: number
  total?: number
  rate: number
  message: string
  error?: string
  warnings: string[]
  artifacts: Artifact[]
  createdAt: number
  updatedAt: number
}

export interface TaskEvent {
  sequence: number
  task: TaskSnapshot
}

export interface AppSettings {
  defaultOutputDir: string
  maxActiveTasks: number
}

export interface ProviderInfo {
  id: string
  name: string
  version: string
  description: string
  enabled: boolean
  capabilities: {
    authentication: boolean
    settings: boolean
  }
}

export interface ProviderAuthPage {
  html: string
  height: number
}

export interface ProviderAuthActionRequest {
  action: string
  payload: unknown
}

export interface ProviderSettingStatus {
  key: string
  label: string
  available: boolean
  value: string
  description: string
}

export interface ProviderSettingAction {
  key: string
  label: string
  description: string
  style: 'primary' | 'secondary' | 'danger'
}

export interface ProviderSettingsSection {
  key: string
  title: string
  description: string
  fields: FormField[]
  statuses: ProviderSettingStatus[]
  actions: ProviderSettingAction[]
}

export interface ProviderSettingsPage {
  html: string
  height: number
}

export interface ProviderSettingsView {
  sections: ProviderSettingsSection[]
  customPage?: ProviderSettingsPage
}

export interface ProviderSettingsState {
  values: Record<string, unknown>
}

export interface ProviderSettingsActionRequest {
  action: string
  values: Record<string, unknown>
}

export interface ProviderSettingsActionResult {
  message: string
  refresh: boolean
}

export interface BinaryAsset {
  contentType: string
  bytes: string
}
