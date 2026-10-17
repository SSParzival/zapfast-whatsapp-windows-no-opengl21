# ZapFast Windows Compat

Un fork personal y sencillo de [ZapFast](https://github.com/crmne/zapfast),
creado para que funcione en este PC sin exigir un contexto OpenGL 2.1.
La adaptación utiliza **Direct3D 12/WARP** en Windows.

Repositorio del fork: <https://github.com/SSParzival/zapfast-windows-compat>.

**Crédito de asistencia de esta adaptación: GPT-6.1 Sol (medium).**
El cliente original es obra de Carmine Paolino y los colaboradores de ZapFast.

El equipo probado usa Windows 11 Pro x64, Intel N95, unos 16 GB de RAM,
controlador básico de pantalla de Microsoft y escritorio 1280x800 al 100 %.
El alcance es la compatibilidad con ese equipo; otros equipos y sistemas
necesitan sus propias comprobaciones.

La versión validada es **0.19.0-dx12.3**. Incluye recuperación al caducar el QR
y mejoras en las listas de chats y la búsqueda de contactos. Conserva el
protocolo, el archivo cifrado y las funciones del proyecto original.

Para compilar y validar en Windows, con Rust y las herramientas C++ instaladas:

```powershell
.\scripts\build-windows-compat.ps1
```

El resultado queda en `dist/windows-compat`. Abre `zapfast.exe` o
`start-software-rendering.cmd` para forzar WARP por CPU. Vincula el teléfono
desde **WhatsApp > Dispositivos vinculados**. Las actualizaciones oficiales
están desactivadas en esta variante para conservar la adaptación gráfica.

Detalles: [compatibilidad](docs/windows-compatibility.md),
[vinculación](docs/pairing-recovery.md), [rendimiento](docs/performance-windows.md)
y [preparación para publicar](FORK.md). Las suites de Windows pasaron con
1.867 y 1.869 pruebas, cero fallos, y siete casos gráficos con datos ficticios.
Los informes distinguen lo probado; la autorización desde el teléfono y el
envío de mensajes reales requieren la comprobación del titular.

Se conserva el [README original](README.upstream.md), la [licencia MIT](LICENSE)
y los [avisos de terceros](THIRD-PARTY-NOTICES.md). Este fork es independiente
del proyecto original, de WhatsApp y de Meta.
