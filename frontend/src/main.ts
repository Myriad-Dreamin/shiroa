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
    if (origin) {
      const onclick = a.getAttribute('onclick');
      if (onclick === null) {
        let target = a.getAttribute('target');
        if (target === '_blank') {
          // remove the target attribute
          a.removeAttribute('target');
        }
      } else if (globalSemaLabels) {
        if (onclick.startsWith('handleTypstLocation')) {
          // get params(p, x, y) in 'handleTypstLocation(this, p, x, y)'
          const [u, x, y] = onclick
            .split('(')[1]
            .split(')')[0]
            .split(',')
            .slice(1)
            .map(s => Number.parseFloat(s.trim()));
          for (const [label, pos] of globalSemaLabels) {
            const [u1, x1, y1] = pos;
            if (u === u1 && Math.abs(x - x1) < 0.01 && Math.abs(y - y1) < 0.01) {
              // todo: deduplicate id
              a.id = `typst-label-${label}`;
              a.setAttribute('href', `#label-${label}`);
              a.setAttribute('xlink:href', `#label-${label}`);
              break;
            }
          }
        }
      }
    }

    // update cross-link
    const decodeTypstUrlc = (s: string) =>
      s
        .split('-')
        .map(s => {
          const n = Number.parseInt(s);
          if (Number.isNaN(n)) {
            return s;
          } else {
            return String.fromCharCode(n);
          }
        })
        .join('');
    const href = a.getAttribute('href')! || a.getAttribute('xlink:href')!;
    if (href.startsWith('cross-link')) {
      const url = new URL(href);
      const pathLabelUnicodes = url.searchParams.get('path-label')!;
      const labelUnicodes = url.searchParams.get('label');
      const plb = decodeTypstUrlc(pathLabelUnicodes).replace('.typ', '.html').replace(/^\//g, '');
      let absolutePath = window.typstPathToRoot ? window.typstPathToRoot.replace(/\/$/g, '') : '';
      absolutePath = new URL(`${absolutePath}/${plb}`, window.location.href).href;
      if (labelUnicodes) {
        absolutePath += '#label-' + encodeURIComponent(decodeTypstUrlc(labelUnicodes));
      }
      a.setAttribute('href', absolutePath);
      a.setAttribute('xlink:href', absolutePath);
      // todo: label handling
    }
  });
}

let prevHovers: Element[] | undefined = undefined;

function updateHovers(elems: Element[]) {
  if (prevHovers) {
    for (const h of prevHovers) {
      h.classList.remove('focus');
    }
  }
  prevHovers = elems;
}
let globalSemaLabels: [string, [number, number, number]][] = [];

window.assignSemaHash = (u: number, x: number, y: number) => {
  // console.log(`find labels ${u}:${x}:${y} in`, globalSemaLabels);
  for (const [label, pos] of globalSemaLabels) {
    const [u1, x1, y1] = pos;
    if (u === u1 && Math.abs(x - x1) < 0.01 && Math.abs(y - y1) < 0.01) {
      location.hash = `label-${label}`;
      const semaLinkLocation = document.getElementById(`typst-label-${label}`);
      const relatedElems: Element[] = window.typstGetRelatedElements(semaLinkLocation);
      for (const h of relatedElems) {
        h.classList.add('focus');
      }
      updateHovers(relatedElems);
      return;
    }
  }
  updateHovers([]);
  // todo: multiple documents
  location.hash = `loc-${u}x${x.toFixed(2)}x${y.toFixed(2)}`;
};

const userAgent = navigator.userAgent.toLowerCase();
const isTablet =
  /(ipad|tablet|(android(?!.*mobile))|(windows(?!.*phone)(.*touch))|kindle|playbook|silk|(puffin(?!.*(IP|AP|WP))))/.test(
    userAgent,
  );

const typstProcessSvgBase = window.typstProcessSvg;
// console.log('isTablet', isTablet, typstProcessSvgBase);
window.typstProcessSvg = function (t: HTMLElement, n: Record<string, any>) {
  n = n || {};
  if (isTablet) {
    n.layoutText = false;
  }
  console.log('layout text feature', n);
  typstProcessSvgBase(t, n);
};

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

  const dec = new TextDecoder();
  reloadArtifact(currTheme).then(() => {
    let initialRender = true;
    const runRender = async () => {
      // const t1 = performance.now();
      // console.log('hold', svgModule, currTheme);

      // todo: bad performance
      appElem.style.margin = `0px`;

      const cached = await plugin.renderToSvg({
        renderSession: svgModule!,
        container: appElem,
      });
      if (!cached) {
        const customs: [string, Uint8Array][] = await plugin.getCustomV1({
          renderSession: svgModule!,
        });
        const semaLabel = customs.find(k => k[0] === 'sema-label');
        if (semaLabel) {
          const labelBin = semaLabel[1];
          const labels = JSON.parse(dec.decode(labelBin));
          globalSemaLabels = labels.map(([label, pos]: [string, string]) => {
            const [_, u, x, y] = pos.split(/[pxy]/).map(Number.parseFloat);
            return [encodeURIComponent(label), [u, x, y]];
          });
        }
      }

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

      if (!cached && window.location.hash) {
        // console.log('hash', window.location.hash);

        // parse location.hash = `loc-${page}x${x.toFixed(2)}x${y.toFixed(2)}`;
        const hash = window.location.hash;
        const firstSep = hash.indexOf('-');
        // console.log('jump label', window.location.hash, firstSep, globalSemaLabels);
        if (firstSep != -1 && hash.slice(0, firstSep) === '#label') {
          const labelTarget = hash.slice(firstSep + 1);
          for (const [label, pos] of globalSemaLabels) {
            if (label === labelTarget) {
              const [u, x, y] = pos;
              // console.log('jump label', label, pos);
              window.handleTypstLocation(appElem.firstElementChild!, u, x, y, {
                behavior: initialRender ? 'smooth' : 'instant',
              });
              initialRender = false;
              break;
            }
          }
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
