
#import "mod.typ": *

#let is-debug = true;

// ---

#head({
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
    raw(lang: "js", read("pollyfill.js")),
    // todo: esm?
    ..styles.final().values(),
  ).join())
  script(
    src: data-url("application/javascript", shiroa-asset-file("shiroa.js", lang: "esm", inline: false).text),
    id: "shiroa-js",
    type: "module",
  )[]

  inline-assets(
    replace-raw(
      // fetch()
      vars: (
        renderer_module: if is-debug {
          data-url(
            "application/wasm",
            read("/assets/artifacts/typst_ts_renderer_bg.wasm", encoding: none),
          )
        } else {
          // todo: path to root
          "/internal/renderer.wasm"
        },
      ),
      ```js
      window.typstRerender = () => { };
      window.typstChangeTheme = () => { };

      var typstBookJsLoaded = new Promise((resolve, reject) => {
          document.getElementById('shiroa-js').addEventListener('load', resolve);
          document.getElementById('shiroa-js').addEventListener('error', reject);
      });

      var rendererWasmModule = fetch('{{ renderer_module }}');
      window.typstBookJsLoaded = typstBookJsLoaded;
      window.typstRenderModuleReady = typstBookJsLoaded.then(() => {
          var typstRenderModule = window.typstRenderModule =
              window.TypstRenderModule.createTypstRenderer();
          return typstRenderModule
              .init({
                  getModule: () => rendererWasmModule,
              }).then(() => typstRenderModule);
      }).catch((err) => {
          console.error('shiroa.js failed to load', err);
      });
      ```,
    ),
  )

  virt-slot("sl:book-meta")
})
