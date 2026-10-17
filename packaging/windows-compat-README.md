# ZapFast para WhatsApp en Windows sin OpenGL 2.1 0.19.0-dx12.3

Fork personal de ZapFast para un PC Windows 11 Pro x64 con Intel N95,
unos 16 GB de RAM y controlador básico de Microsoft.
Crédito de asistencia de esta adaptación: GPT-6.1 Sol (medium).
ZapFast original: Carmine Paolino y colaboradores, https://github.com/crmne/zapfast.
Repositorio del fork: https://github.com/SSParzival/zapfast-whatsapp-windows-sin-opengl21.

Abre `zapfast.exe`. Windows dibuja con Direct3D 12 sin exigir un contexto
OpenGL 2.1. `start-software-rendering.cmd` fuerza WARP por CPU.

Vincula el teléfono desde WhatsApp > Dispositivos vinculados. Si el QR caduca,
pulsa Reintentar; la recuperación conserva la sesión y el archivo local.
La autorización de la cuenta se completa en el teléfono.

La variante conserva las funciones del cliente original y mejora las listas
de chats y la búsqueda de contactos. Las actualizaciones oficiales están
desactivadas para mantener la adaptación gráfica. Actualiza esta variante
manualmente desde el repositorio del fork.

Consulta `windows-compatibility.md`, `pairing-recovery.md` y
`performance-windows.md` para ver cambios, pruebas y límites. La recepción y
renovación del QR se comprobaron con un perfil temporal en .2; esta versión
conserva ese código. Los casos gráficos y las mediciones usan datos ficticios.
El envío de mensajes reales requiere la comprobación del titular.

Se conservan `LICENSE` y `THIRD-PARTY-NOTICES.md`. Este fork es independiente
del proyecto original, de WhatsApp y de Meta.
