# 🦀 Buscaminas v2.2.2 (Rust · Windows & macOS)

Versión del clásico **Buscaminas** reescrita en **Rust** con Macroquad. El mismo código fuente se compila de forma nativa para **Windows** y **macOS**, incluyendo un binario universal para **Apple Silicon e Intel**.

## Descargas

Las versiones compiladas se publican exclusivamente en **GitHub Releases**; el repositorio mantiene sólo código fuente y recursos de construcción.

### Windows

Descarga `Buscaminas_v2.exe` desde Releases y ejecútalo directamente. No requiere instalar Rust ni un runtime adicional.

Los récords se guardan en `%LOCALAPPDATA%\Buscaminas\mejores_tiempos_v2.json`.

### macOS

Descarga `Buscaminas-macOS-Universal.zip`, descomprímelo y abre `Buscaminas.app`.

El artefacto generado por GitHub Actions contiene un binario universal `arm64 + x86_64`. Actualmente el workflow aplica una **firma ad-hoc** para comprobar la integridad del bundle, pero la aplicación **no está notarizada por Apple ni firmada con un certificado Developer ID**. Por ello Gatekeeper puede bloquear la primera apertura de una copia descargada de Internet. Para una distribución pública sin avisos de Gatekeeper se necesita un certificado Apple Developer ID y notarización de Apple.

Los récords se guardan en `~/Library/Application Support/Buscaminas/mejores_tiempos_v2.json`.

## Características

- Principiante: 9×9, 10 minas.
- Intermedio: 16×16, 40 minas.
- Experto: 30×16, 99 minas.
- Modo personalizado.
- Primer clic seguro.
- Chording mediante botón central o combinación izquierda + derecha.
- Contadores LED de minas y tiempo.
- Efectos de partículas y screen shake.
- Audio procedural generado en memoria.
- Si el subsistema de audio no puede inicializarse, el juego continúa funcionando sin sonido.
- Tabla de mejores tiempos persistente en JSON.

## Compilar desde código

Necesitas una instalación reciente de Rust.

```bash
cargo build --locked --release
```

Para ejecutar durante el desarrollo:

```bash
cargo run --locked --release
```


## Calidad y CI

Cada pull request y actualización de `main` ejecuta comprobaciones automáticas antes de generar los builds nativos. La CI comprueba compilación con el `Cargo.lock`, exige formato `rustfmt`, ejecuta Clippy con los warnings tratados como errores y ejecuta los tests.

En macOS la CI compila por separado para:

- `aarch64-apple-darwin`
- `x86_64-apple-darwin`

Después combina ambas arquitecturas con `lipo`, valida el `Info.plist`, verifica el bundle con `codesign` y genera `Buscaminas.app`.

## Código fuente

El juego no necesita recursos externos en tiempo de ejecución para sus sonidos. Las imágenes de icono del repositorio se utilizan únicamente durante el empaquetado de las aplicaciones.
