{{/*
Expand the name of the chart.
*/}}
{{- define "xkcd-explorer.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "xkcd-explorer.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "xkcd-explorer.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "xkcd-explorer.labels" -}}
helm.sh/chart: {{ include "xkcd-explorer.chart" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Server Selector labels
*/}}
{{- define "xkcd-explorer.serverSelectorLabels" -}}
app.kubernetes.io/name: {{ include "xkcd-explorer.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "xkcd-explorer.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "xkcd-explorer.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Return the TorchServe server Name
*/}}
{{- define "xkcd-explorer.torchserveName" -}}
{{ printf "%s-torchserve" (include "xkcd-explorer.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end -}}

{{/*
TorchServe Selector labels
*/}}
{{- define "xkcd-explorer.torchserveSelectorLabels" -}}
app.kubernetes.io/name: {{ include "xkcd-explorer.torchserveName" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Return the Gradio Demo Name
*/}}
{{- define "xkcd-explorer.gradioDemoName" -}}
{{ printf "%s-gradio-demo" (include "xkcd-explorer.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end -}}

{{/*
Gradio Demo Selector labels
*/}}
{{- define "xkcd-explorer.gradioDemoSelectorLabels" -}}
app.kubernetes.io/name: {{ include "xkcd-explorer.gradioDemoName" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Return the Demo Data Job Name
*/}}
{{- define "xkcd-explorer.demoDataJobName" -}}
{{ printf "%s-demo-data-job" (include "xkcd-explorer.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end -}}

{{/*
Demo Data Job Selector labels
*/}}
{{- define "xkcd-explorer.demoDataJobSelectorLabels" -}}
app.kubernetes.io/name: {{ include "xkcd-explorer.demoDataJobName" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}