//#/dev/frontend/dist/book.mjs
import { kObject } from '@myriaddreamin/typst.ts/dist/esm/internal.types.mjs';
import {
  RenderSession,
  TypstRenderer,
  createTypstRenderer,
} from '@myriaddreamin/typst.ts/dist/esm/renderer.mjs';

window.TypstRenderModule = {
  createTypstRenderer,
};

// window.debounce = function debounce<T extends { (...args: any[]): void }>(fn: T, delay = 200) {
//   let timer: number;

//   return ((...args: any[]) => {
//     clearTimeout(timer);
//     timer = setTimeout(() => {
//       fn(...args);
//     }, delay);
//   }) as unknown as T;
// };
function postProcessCrossLinks(appElem: HTMLDivElement) {
  appElem.querySelectorAll('.pseudo-link').forEach(link => {
    // update target
    const a = link.parentElement!;
    if (origin && a.getAttribute('onclick') === null) {
      let target = a.getAttribute('target');
      if (target === '_blank') {
        // remove the target attribute
        a.removeAttribute('target');
      }
    }

    // update cross-link
    const href = a.getAttribute('href')! || a.getAttribute('xlink:href')!;
    if (href.startsWith('cross-link')) {
      const url = new URL(href);
      const pathLabelUnicodes = url.searchParams.get('path-label')!;
      const plb = pathLabelUnicodes
        .split('-')
        .map(s => {
          const n = Number.parseInt(s);
          if (Number.isNaN(n)) {
            return s;
          } else {
            return String.fromCharCode(n);
          }
        })
        .join('')
        .replace('.typ', '.html');
      const absolutePath = new URL(plb, window.location.href).href;
      a.setAttribute('href', absolutePath);
      a.setAttribute('xlink:href', absolutePath);
      // todo: label handling
    }
  });
}

window.typstBookRenderPage = function (
  plugin: TypstRenderer,
  relPath: string,
  appContainer: HTMLDivElement | undefined,
) {
  // todo: preload artifact
  const getTheme = () => window.getTypstTheme();
  let currTheme = getTheme();
  let svgModule: RenderSession | undefined = undefined;

  const appElem = document.createElement('div');
  if (appElem && appContainer) {
    appElem.className = 'typst-app';
    appContainer.appendChild(appElem);
  }

  async function reloadArtifact(theme: string) {
    // free anyway
    if (svgModule) {
      try {
        (svgModule as any)[kObject].free();
      } catch (e) {}
    }

    appElem.innerHTML = '';
    // todo: don't modify this attribute here, instead hide detail in typst.ts
    appElem.removeAttribute('data-applied-width');

    const t0 = performance.now();
    const artifactData = await fetch(`${relPath}.${theme}.multi.sir.in`)
      .then(response => response.arrayBuffer())
      .then(buffer => new Uint8Array(buffer));
    const t1 = performance.now();
    svgModule = (await plugin.createModule(artifactData)) as RenderSession;
    const t2 = performance.now();

    console.log(
      `theme = ${theme}, load artifact took ${t2 - t1} milliseconds, parse artifact took ${
        t2 - t1
      } milliseconds`,
    );
  }

  reloadArtifact(currTheme).then(() => {
    const runRender = async () => {
      // const t1 = performance.now();
      // console.log('hold', svgModule, currTheme);

      // todo: bad performance
      appElem.style.margin = `0px`;

      await plugin.renderToSvg({
        renderSession: svgModule!,
        container: appElem,
      });

      postProcessCrossLinks(appElem);

      // const t2 = performance.now();
      // console.log(
      //   `render took ${t2 - t1} milliseconds.`,
      //   appElem.getAttribute('data-applied-width'),
      // );

      const w = appElem.getAttribute('data-applied-width');
      if (w) {
        const parentWidth = appElem.parentElement!.clientWidth;
        const svgWidth = Number.parseInt(w.slice(0, w.length - 2));
        // console.log(
        //   parentWidth,
        //   svgWidth,
        //   window.devicePixelRatio,
        //   getComputedStyle(appElem).fontSize,
        // );
        const wMargin = (parentWidth - svgWidth) / 2;
        if (wMargin < 0) {
          appElem.style.margin = `0px`;
        } else {
          appElem.style.margin = `0 ${wMargin}px`;
        }
      }
    };

    let base = runRender();

    window.typstRerender = () => {
      return (base = base.then(runRender));
    };

    window.typstChangeTheme = () => {
      const nextTheme = getTheme();
      if (nextTheme === currTheme) {
        return base;
      }
      currTheme = nextTheme;

      return (base = base.then(() => reloadArtifact(currTheme).then(runRender)));
    };

    window.onresize = window.typstRerender;

    // trigger again to regard user changed theme during first reloading
    window.typstChangeTheme();
  });
};
