@title Ejemplo Completo de Documentación  

@paragraph Este archivo demuestra el uso de todos los decoradores disponibles para generar documentación HTML de forma estructurada, limpia y comprensible.  

@-  

@subtitle Información General  

@info Esta sección explica cómo utilizar correctamente cada decorador dentro del sistema de documentación.  

@paragraph Los decoradores permiten transformar texto en etiquetas HTML automáticamente, facilitando la creación de documentación técnica.  

@-  

@subtitle Lista de Comandos  

@list  
@item @title → Crea un título principal.  
@item @subtitle → Crea un subtítulo.  
@item @paragraph → Agrega un párrafo descriptivo.  
@item @list y @endlist → Inician y cierran una lista.  
@item @item → Define un elemento dentro de una lista.  
@item @code y @endcode → Muestran bloques de código formateado.  
@item @info → Muestra un recuadro informativo.  
@item @warn → Muestra un recuadro de advertencia.  
@item @link → Crea un enlace externo.  
@item @image → Inserta una imagen.  
@item @quote → Muestra una cita o frase destacada.  
@item @small → Agrega texto en tamaño pequeño.  
@endlist  

@-  

@subtitle Ejemplo de Código  

@code  
function saludo() {
  console.log("Hola, mundo!");
}
@endcode  

@-  

@subtitle Ejemplo de Enlace e Imagen  

@paragraph Puedes visitar el siguiente enlace para más información:  
@link https://developer.mozilla.org/es/

@paragraph También puedes incluir imágenes ilustrativas:  
@image https://placehold.co/600x400 

@-  

@subtitle Ejemplo de Mensajes  

@info Este es un mensaje informativo dentro de un recuadro azul.  
@warn Este es un mensaje de advertencia que resalta información importante.  

@-  

@subtitle Cita y Texto Pequeño  

@quote "El código limpio siempre se escribe dos veces." — Robert C. Martin  

@small Documento generado automáticamente con decoradores personalizados.  

@-  

@* Fin del ejemplo de demostración de decoradores HTML personalizados.  
