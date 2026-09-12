# 🦀 Buscaminas v2.4.0 (Rust · Windows, macOS & Linux)

Versión del clásico **Buscaminas** reescrita en **Rust** con Macroquad. El mismo código fuente se compila de forma nativa para **Windows**, **macOS** y **Linux**, incluyendo un binario universal para **Apple Silicon e Intel**.

## Descargas

Las versiones compiladas se publican exclusivamente en **GitHub Releases**; el repositorio mantiene sólo código fuente y recursos de construcción.

### Windows

Descarga `Buscaminas_v2.exe` desde Releases y ejecútalo directamente. No requiere instalar Rust ni un runtime adicional.

Los récords se guardan en `%LOCALAPPDATA%\Buscaminas\mejores_tiempos_v2.json` y los ajustes en `ajustes_v2.json`, en esa misma carpeta.

### macOS

Descarga `Buscaminas-macOS-Universal.zip`, descomprímelo y abre `Buscaminas.app`.

El artefacto generado por GitHub Actions contiene un binario universal `arm64 + x86_64`. Actualmente el workflow aplica una **firma ad-hoc** para comprobar la integridad del bundle, pero la aplicación **no está notarizada por Apple ni firmada con un certificado Developer ID**. Por ello Gatekeeper puede bloquear la primera apertura de una copia descargada de Internet. Para una distribución pública sin avisos de Gatekeeper se necesita un certificado Apple Developer ID y notarización de Apple.

Los récords se guardan en `~/Library/Application Support/Buscaminas/mejores_tiempos_v2.json` y los ajustes en `ajustes_v2.json`, en esa misma carpeta.

### Linux

Descarga `Buscaminas-Linux-x86_64.tar.gz`, descomprímelo y ejecuta el binario:

```bash
tar -xzf Buscaminas-Linux-x86_64.tar.gz
./buscaminas
```

El binario se compila en Ubuntu 22.04, que enlaza contra una glibc más antigua que la de las versiones actuales, de modo que vale en más distribuciones. Necesita en tiempo de ejecución:

- **X11** (`libX11.so.6`) y **OpenGL** (`libGL.so.1`), que miniquad carga con `dlopen` al arrancar. En Wayland funciona a través de XWayland.
- **ALSA** (`libasound.so.2`), presente en cualquier escritorio con audio. Si no está o no hay tarjeta de sonido, el juego arranca igual y se queda sin sonido.

Los récords se guardan en `$XDG_DATA_HOME/buscaminas/mejores_tiempos_v2.json` y los ajustes en `ajustes_v2.json`, en esa misma carpeta. Si `XDG_DATA_HOME` no está definida se usa `~/.local/share/buscaminas`.

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
- Dos temas de música de fondo, compuestos y sintetizados en código: **relax** (fa mayor, bucle de 64 s) y **concentración** (la menor, bucle de 80 s). Se eligen en *Opciones*, y el juego arranca con el primero.
- Si el subsistema de audio no puede inicializarse, el juego continúa funcionando sin sonido.
- Tabla de mejores tiempos persistente en JSON.
- Ajustes persistentes: música, sonido, partículas y marcas (?).

## Compilar desde código

Necesitas una instalación reciente de Rust.

```bash
cargo build --locked --release
```

Para ejecutar durante el desarrollo:

```bash
cargo run --locked --release
```

Para volcar los dos temas a WAV y escucharlos fuera del juego:

```bash
cargo run --locked --release --example exportar_musica -- carpeta_destino
```


## Calidad y CI

Cada pull request y actualización de `main` ejecuta comprobaciones automáticas antes de generar los builds nativos. La CI comprueba compilación con el `Cargo.lock`, exige formato `rustfmt`, ejecuta Clippy con los warnings tratados como errores y ejecuta los tests.

En macOS la CI compila por separado para:

- `aarch64-apple-darwin`
- `x86_64-apple-darwin`

Después combina ambas arquitecturas con `lipo`, valida el `Info.plist`, verifica el bundle con `codesign` y genera `Buscaminas.app`.

En Linux, además de compilar, la CI arranca el binario resultante bajo `Xvfb` durante 15 segundos —sin ventana real ni tarjeta de sonido— y falla si no se mantiene en ejecución.

## Código fuente

El juego no necesita recursos externos en tiempo de ejecución para su audio: tanto los efectos como los dos temas de música se generan por síntesis en memoria, sin samples ni ficheros de sonido en el repositorio. Las imágenes de icono del repositorio se utilizan únicamente durante el empaquetado de las aplicaciones.
