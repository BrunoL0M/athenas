# Athenas

> La memoria externa de tu vida como developer.

Athenas es una aplicación de escritorio **local-first** (Tauri 2 + Rust + React) que centraliza el contexto de tus proyectos personales: qué stack usan, dónde quedó tu última sesión de trabajo, qué estado tiene el repo git, y qué proyectos olvidados existen en tu disco.

No es un gestor de tareas ni un dashboard de observabilidad — **gestiona tu contexto**: sabe dónde quedaste, qué olvidaste y cómo evolucionaste como dev.

Nombre: del griego "Atenea" (Athena), diosa de la sabiduría — referencia a la idea de ser tu "memoria externa" y "guía" de desarrollo.

## Features (roadmap)

- 🔍 **Disk archaeology** — escanea tu directorio de desarrollo y detecta todos los proyectos, incluidos los olvidados
- 🧠 **Git intelligence** — branch actual, commits sin pushear, actividad reciente, heatmap local
- 📌 **"Dónde quedé"** — snapshots de sesión para retomar el contexto tras semanas
- 🩺 **Health check** — dependencias desactualizadas, `node_modules`/`target` ocupando gigas, `.env` fuera de `.gitignore`
- 🚀 **Quick launch** — abrir en editor, terminal o carpeta con un click

## Stack

- **Backend:** Rust (Tauri 2, rusqlite, git2, walkdir)
- **Frontend:** React 19 + TypeScript, TailwindCSS 4, Zustand
- **Datos:** SQLite local — sin servidor, sin cuentas, sin sync remoto

## Desarrollo

```bash
pnpm install
pnpm tauri dev
```

## Estado

🚧 En desarrollo activo — fase inicial (MVP). Plataforma objetivo inicial: Linux (Wayland), luego multiplataforma vía Tauri.

## Licencia

[MIT](LICENSE)
