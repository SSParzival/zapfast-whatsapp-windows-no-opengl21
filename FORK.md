# Publicar este fork

Nombre: **ZapFast para WhatsApp en Windows sin OpenGL 2.1** (`zapfast-whatsapp-windows-sin-opengl21`).
Repositorio: <https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21>.
Descripción: «Fork personal de ZapFast para Windows 11 con Intel N95 y
controlador básico de Microsoft, usando DX12/WARP sin exigir OpenGL 2.1».

Crédito de asistencia para las modificaciones de esta adaptación:
**GPT-6.1 Sol (medium)**. Este crédito se refiere al fork. El cliente original
es obra de Carmine Paolino y los colaboradores de ZapFast.

## Procedencia y alcance

- Proyecto original: <https://github.com/crmne/zapfast>.
- Base: `021c568639159b8dca7a76eb4adc6f5d8cac5a11`, paquete 0.19.0 con trabajo
  upstream aún no publicado en ese momento.
- Variante comprobada: `0.19.0-dx12.3`, Windows x64. Los cambios de gráficos,
  recuperación de vinculación y rendimiento están documentados en `docs/`.
- Adaptación personal para el PC descrito en el README, con el protocolo,
  las funciones y la licencia del proyecto original.
- Las pruebas reciben y renuevan un QR; autorizar la cuenta y enviar mensajes
  reales requieren la comprobación del titular. Las mediciones de rendimiento
  son sintéticas y utilizan un ejecutable de pruebas en modo debug.

## Repositorio de destino

Este repositorio es un fork de `crmne/zapfast` en la cuenta `SSParzival`.
La rama `main` conserva el historial original y fija el árbol de código
validado para este PC, basado en `021c568639159b8dca7a76eb4adc6f5d8cac5a11`.
Los cambios posteriores de upstream se mantienen en el historial, sin
incorporarlos al código de esta adaptación. La publicación añade un commit
normal sobre la rama del fork; no necesita reescribir su historial remoto.

Esta copia local conserva el remoto original como `upstream` y el fork
como `origin`. Para obtener otra copia:

```powershell
git clone https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21.git
cd zapfast-whatsapp-windows-sin-opengl21
git remote add upstream https://github.com/crmne/zapfast.git
git remote -v
```

Conserva los cambios de esta adaptación en `main` y el historial existente, sin forzar
ni reescribir commits de upstream. Mantén `LICENSE`, `THIRD-PARTY-NOTICES.md`
y los avisos de los recursos distribuidos.

## Publicación sencilla

Las automatizaciones heredadas se conservan inactivas en
`.github/upstream-workflows/`. Compila y valida con
`scripts/build-windows-compat.ps1`; este fork utiliza publicación manual.

Para distribuir un ejecutable, crea una publicación manual en Releases de tu
fork después de subir a `main` el código validado. Identifica la versión local,
el equipo probado y los límites de validación. Empaqueta el contenido de
`dist/windows-compat` y adjunta el ZIP con su SHA256.

El ejecutable validado de esta versión conserva este SHA256:

```text
1eddcb1af1e957d0766b38154ba1ea31a6fbad9a1ad064ee797dc5b075023235
```

Incluye únicamente el ejecutable, el lanzador, README, licencias, informes
públicos y checksums. Cachés, respaldos, datos de cuentas, registros y capturas
locales quedan fuera del historial. Revisa los archivos antes de añadirlos al
commit, incluso si existen exclusiones en `.gitignore`. Las capturas adjuntas
a una publicación deben mostrar solo datos ficticios.

La documentación web y las recetas históricas de upstream se conservan como
referencia; sus descargas corresponden al proyecto original. Para esta
adaptación, utiliza el README y los Releases del fork.
