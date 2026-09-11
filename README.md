# 🦀 Buscaminas v2 (Edición Rust)

Segunda versión del clásico **Buscaminas** para Windows, reescrita íntegramente desde cero en **Rust** para lograr máximo rendimiento nativo, consumo mínimo de memoria RAM (< 15 MB), aceleración por hardware a 60+ FPS y un ejecutable único standalone de menos de 1 MB.

---

## 🚀 Cómo Jugar

Simplemente haz doble clic en:
- **`Buscaminas_v2.exe`** (incluido en la raíz del repositorio o en la sección de Releases).

No requiere tener Rust ni Python instalados para jugar, y al ser una aplicación GUI pura de Windows, no abre ninguna consola negra.

Si deseas ejecutar desde el código fuente con Cargo:
```bash
cargo run --release
```

---

## ⚡ Mejoras y Novedades de la v2 (Rust)

1. **Rendimiento Nativo Extremo**:
   - Compilado en código máquina optimizado x86_64 con el compilador oficial de Rust.
   - Tiempo de arranque instantáneo (< 5 ms).
   - Tamaño del ejecutable: **< 1 MB** (frente a los ~29 MB de empaquetados tradicionales).
2. **Gráficos Acelerados por Hardware (GPU)**:
   - Renderizado 2D fluido con aceleración gráfica directa.
3. **Efectos Visuales v2**:
   - **Sistema de Partículas**:
     - 🎉 Lluvia de confeti de celebración al ganar la partida.
     - 💥 Explosión con dispersión de fragmentos al pisar una mina.
   - **Screen Shake (Temblor de pantalla)**: Breve sacudida dinámica de la interfaz al detonar una mina.
   - Tecla rápida `P` o menú *Opciones* para activar/desactivar partículas si lo prefieres.
4. **Audio Nativo de Windows**:
   - Sintetizador procedural en memoria integrado con la API multimedia de Windows (`winmm / PlaySoundW`) con reproducción asíncrona sin latencia.
5. **Mecánicas Clásicas Fieles**:
   - **Primer clic seguro garantizado**: Nunca perderás en tu primera jugada; siempre abre una isla inicial despejada.
   - **Chording profesional**: Usa el botón central del ratón, o el doble clic, o ambos botones (izq + der) a la vez en un número para despejar casillas adyacentes rápidamente.
   - Contadores digitales **LED de 7 segmentos** para minas y cronómetro.
   - Carita interactiva (**Smiley**): 🙂 normal, 😮 tensión al mantener clic, 😎 victoria con gafas de sol, 😵 derrota con ojos en cruz.
6. **Dificultades y Récords**:
   - Principiante (9×9, 10 minas), Intermedio (16×16, 40 minas), Experto (30×16, 99 minas) y Personalizado.
   - Tabla de mejores tiempos con persistencia JSON (`mejores_tiempos_v2.json`).

---

## 🛠️ Recompilación

Para volver a compilar el proyecto en modo release:
```bash
compilar_v2.bat
```
O con Cargo:
```bash
cargo build --release
```
