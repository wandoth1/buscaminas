# 🍎 Buscaminas v2.2 para macOS

La versión de macOS se genera como **Universal Binary**, compatible con **Apple Silicon** e **Intel**.

## Instalación

1. Descarga `Buscaminas-macOS-Universal.zip` desde GitHub Releases.
2. Descomprime el archivo.
3. Mueve `Buscaminas.app` a `/Applications` si quieres instalarlo.
4. Abre la aplicación.

## Gatekeeper

El workflow público aplica una firma ad-hoc para verificar la integridad del bundle, pero la aplicación no está firmada con **Developer ID** ni notarizada por Apple. macOS puede bloquear inicialmente una copia descargada de Internet.

Para una distribución pública sin avisos de Gatekeeper es necesario configurar un certificado Apple Developer ID y notarización en GitHub Actions.

## Ejecutar desde código

Si tienes Rust instalado:

```bash
cargo run --release
```

Los mejores tiempos se guardan en:

```text
~/Library/Application Support/Buscaminas/mejores_tiempos_v2.json
```
