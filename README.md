# 📘 Documentación de decoradores `.jol`

Los decoradores son comandos que transforman texto en estructuras HTML al ejecutar el archivo con:

```
run -c /ruta/al/archivo.jol
```

Cada decorador inicia con `@` y genera una etiqueta HTML específica.

---

## 🏷️ Decoradores de estructura

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@title` | Crea un título principal. | `<h1>{contenido}</h1>` |
| `@subtitle` | Crea un subtítulo. | `<h3>{contenido}</h3>` |
| `@paragraph` | Genera un párrafo. | `<p>{contenido}</p>` |
| `@-` | Inserta una línea divisoria. | `<hr/>` |
| `@*` | Inserta un comentario HTML. | `<!-- {contenido} -->` |

---

## 📋 Listas

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@list` | Inicia una lista. | `<ul>` |
| `@endlist` | Finaliza una lista. | `</ul>` |
| `@item` | Agrega un elemento a la lista. | `<li>{contenido}</li>` |

---

## 💻 Código

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@code` | Inicia un bloque de código. | `<pre><code>` |
| `@endcode` | Finaliza el bloque de código. | `</code></pre>` |

---

## ⚠️ Cajas de información

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@info` | Muestra un cuadro informativo. | `<div class='info-box'>{contenido}</div>` |
| `@warn` | Muestra una advertencia. | `<div class='warn-box'>{contenido}</div>` |

---

## 🔗 Enlaces e imágenes

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@link` | Crea un enlace externo. | `<a href='{contenido}' target='_blank'>{contenido}</a>` |
| `@image` | Inserta una imagen. | `<img src='{contenido}' alt='image'/>` |

---

## 💬 Otros

| Decorador | Descripción | HTML generado |
|------------|--------------|----------------|
| `@quote` | Crea una cita o bloque destacado. | `<blockquote>{contenido}</blockquote>` |
| `@small` | Muestra texto en tamaño reducido. | `<small>{contenido}</small>` |
