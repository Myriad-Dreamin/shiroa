//#/dev/frontend/dist/book.mjs
import { SvgSession } from '@myriaddreamin/typst-ts-renderer';
import {
  TypstSvgRenderer,
  createTypstSvgRenderer,
} from '@myriaddreamin/typst.ts/dist/esm/renderer';

window.TypstRenderModule = {
  createTypstSvgRenderer,
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

window.typstBookRenderPage = function (
  plugin: TypstSvgRenderer,
  relPath: string,
  appContainer: HTMLDivElement | undefined,
) {
  // todo: preload artifact
  const getTheme = () => window.getTypstTheme();
  let currTheme = getTheme();
  let svgModule: SvgSession | undefined = undefined;

  const appElem = document.createElement('div');
  if (appElem && appContainer) {
    appElem.className = 'typst-app';
    appContainer.appendChild(appElem);
  }

  async function reloadArtifact(theme: string) {
    // free anyway
    if (svgModule) {
      try {
        svgModule.free();
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
    svgModule = (await plugin.createModule(artifactData)) as SvgSession;
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

      // todo: merge
      await (plugin as any).renderSvg(svgModule, appElem);

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
