# ZapFast para WhatsApp en Windows sin OpenGL 2.1

**Cliente nativo y no oficial de WhatsApp para Windows 11, adaptado para
funcionar con el controlador básico de pantalla de Microsoft.**

**Cambio principal:** el motor gráfico de Windows pasa a **Direct3D 12/WARP**,
eliminando la exigencia de un contexto **OpenGL 2.1** de la versión base.
WARP permite dibujar mediante la CPU del PC cuando el controlador gráfico
disponible no ofrece el soporte necesario para esa versión de OpenGL.

Es un fork personal de [ZapFast](https://github.com/crmne/zapfast), preparado
y comprobado para el equipo descrito abajo.

Repositorio del fork: <https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21>.

**Crédito de asistencia de esta adaptación: GPT-6.1 Sol (medium).**
El cliente original es obra de Carmine Paolino y los colaboradores de ZapFast.

El equipo probado usa Windows 11 Pro x64, Intel N95, unos 16 GB de RAM,
controlador básico de pantalla de Microsoft y escritorio 1280x800 al 100 %.
El alcance es la compatibilidad con ese equipo; otros equipos y sistemas
necesitan sus propias comprobaciones.

La versión validada es **0.19.0-dx12.3**. Incluye recuperación al caducar el QR
y mejoras en las listas de chats y la búsqueda de contactos. Conserva el
protocolo, el archivo cifrado y las funciones del proyecto original.

**[Descargar ZIP para Windows x64](https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21/releases/download/v0.19.0-dx12.3/ZapFast-0.19.0-dx12.3-windows-x64.zip)**
· [SHA256](https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21/releases/download/v0.19.0-dx12.3/ZapFast-0.19.0-dx12.3-windows-x64.zip.sha256)
· [Notas y capturas](https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21/releases/tag/v0.19.0-dx12.3).
La publicación se identifica como una versión preliminar de esta adaptación.
Extrae el ZIP y abre `zapfast.exe`.

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
