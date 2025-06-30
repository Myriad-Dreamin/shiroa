
#import "mod.typ": *

#let is-debug = false;

// ---

#head({
  [#metadata[] <keep-html>]
  meta(charset: "utf-8")
  virt-slot("meta-title")
  meta(
    name: "viewport",
    content: "width=device-width, initial-scale=1.0",
  )
  meta(name: "generator", content: "Shiroa")
  // todo: <!-- Custom HTML head -->
  // todo: auto description.
  meta(name: "theme-color", content: "#ffffff")
  // todo: favicon.png

  inline-assets(context (
    raw(lang: "css", read("css/chrome.css")),
    raw(lang: "css", read("css/general.css")),
    raw(lang: "css", read("css/variables.css")),
    raw(lang: "css", read("FontAwesome/css/font-awesome.css")),
    if is-debug {
      raw(lang: "js", read("/assets/artifacts/elasticlunr.min.js"))
      raw(lang: "js", read("/assets/artifacts/mark.min.js"))
      raw(lang: "js", read("/assets/artifacts/searcher.js"))
    },
    raw(lang: "js", read("pollyfill.js")),
    ..styles.final().values(),
  ).join())

  // <script id="shiroa-js" type="module" src="{{ path_to_root }}internal/shiroa.js"></script>
  // {{!-- <script id="shiroa-js" type="module" src="/dev/frontend/dist/book.mjs"></script> --}}
  // <script>
  //     window.typstRerender = () => { };
  //     window.typstChangeTheme = () => { };

  //     var typstBookJsLoaded = new Promise((resolve, reject) => {
  //         document.getElementById('shiroa-js').addEventListener('load', resolve);
  //         document.getElementById('shiroa-js').addEventListener('error', reject);
  //     });

  //     var rendererWasmModule = fetch('{{ renderer_module }}');
  //     {{!-- var rendererWasmModule = fetch('/dev/frontend/node_modules/@myriaddreamin/typst-ts-renderer/pkg/typst_ts_renderer_bg.wasm'); --}}
  //     window.typstBookJsLoaded = typstBookJsLoaded;
  //     window.typstRenderModuleReady = typstBookJsLoaded.then(() => {
  //         var typstRenderModule = window.typstRenderModule =
  //             window.TypstRenderModule.createTypstRenderer();
  //         return typstRenderModule
  //             .init({
  //                 getModule: () => rendererWasmModule,
  //             }).then(() => typstRenderModule);
  //     }).catch((err) => {
  //         console.error('shiroa.js failed to load', err);
  //     });
  // </script>

  virt-slot("sl:book-meta")
})
