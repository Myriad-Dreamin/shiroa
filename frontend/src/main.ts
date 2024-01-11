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
  appElem.querySelectorAll('.typst-content-link').forEach(a => {
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
          for (const [label, _dom, pos] of globalSemaLabels) {
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
let globalSemaLabels: [string, SVGSVGElement, [number, number, number]][] = [];

function findLinkInSvg(r: SVGSVGElement, xy: [number, number]) {
  // children
  const bbox = r.getBoundingClientRect();
  if (
    xy[0] < bbox.left - 1 ||
    xy[0] > bbox.right + 1 ||
    xy[1] < bbox.top - 1 ||
    xy[1] > bbox.bottom + 1
  ) {
    return;
  }

  // foreignObject
  if (r.classList.contains('pseudo-link')) {
    return r;
  }

  for (let i = 0; i < r.children.length; i++) {
    const a = findLinkInSvg(r.children[i] as any as SVGSVGElement, xy) as SVGAElement;
    if (a) {
      return a;
    }
  }

  return undefined;
}

const findAncestor = (el: Element, cls: string) => {
  while (el && !el.classList.contains(cls)) el = el.parentElement!;
  return el;
};

window.typstBookRenderPage = function (
  plugin: TypstRenderer,
  relPath: string,
  appContainer: HTMLDivElement | undefined,
) {
  // todo: preload artifact
  const getTheme = () => window.getTypstTheme();
  let currTheme = getTheme();
  let session: RenderSession | undefined = undefined;
  let disposeSession: () => void = () => {};

  const appElem = document.createElement('div');
  if (appElem && appContainer) {
    appElem.className = 'typst-app';
    appContainer.appendChild(appElem);
  }

  function getViewport() {
    const domScale = 1;
    const appPos = appElem.getBoundingClientRect();
    const left = appPos.left;
    const top = -appPos.top;
    const right = window.innerWidth;
    const bottom = window.innerHeight - appPos.top;
    const rect = {
      x: 0,
      y: top / domScale,
      width: Math.max(right - left, 0) / domScale,
      height: Math.max(bottom - top, 0) / domScale,
    };
    if (rect.width <= 0 || rect.height <= 0) {
      rect.x = rect.y = rect.width = rect.height = 0;
    }
    return rect;
  }

  const dec = new TextDecoder();
  window.typstBindSvgDom = async (elem: HTMLDivElement, dom: SVGSVGElement) => {};

  let initialRender = true;
  const typstBindCustomSemantics = async (
    root: HTMLElement,
    svg: SVGSVGElement,
    semantics: HTMLDivElement,
  ) => {
    console.log('bind custom semantics', root, svg, semantics);
    const customs: [string, Uint8Array][] = await plugin.getCustomV1({
      renderSession: session!,
    });
    const semaLabel = customs.find(k => k[0] === 'sema-label');
    if (semaLabel) {
      const labelBin = semaLabel[1];
      const labels = JSON.parse(dec.decode(labelBin));
      globalSemaLabels = labels.map(([label, pos]: [string, string]) => {
        const [_, u, x, y] = pos.split(/[pxy]/).map(Number.parseFloat);
        return [encodeURIComponent(label), svg, [u, x, y]];
      });
    }

    postProcessCrossLinks(semantics);

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

    // todo: out of page
    if (window.location.hash) {
      // console.log('hash', window.location.hash);

      // parse location.hash = `loc-${page}x${x.toFixed(2)}x${y.toFixed(2)}`;
      const hash = window.location.hash;
      const firstSep = hash.indexOf('-');
      // console.log('jump label', window.location.hash, firstSep, globalSemaLabels);
      if (firstSep != -1 && hash.slice(0, firstSep) === '#label') {
        const labelTarget = hash.slice(firstSep + 1);
        for (const [label, dom, pos] of globalSemaLabels) {
          if (label === labelTarget) {
            const [_, x, y] = pos;
            // console.log('jump label', label, pos);
            window.handleTypstLocation(dom, 1, x, y, {
              behavior: initialRender ? 'smooth' : 'instant',
            });
            initialRender = false;
            break;
          }
        }
      }
    }
  };
  (window as any).typstBindCustomSemantics = (
    root: HTMLElement,
    svg: SVGSVGElement,
    semantics: HTMLDivElement,
  ) => setTimeout(() => typstBindCustomSemantics(root, svg, semantics), 0);

  const baseHandleTypstLocation = window.handleTypstLocation;
  window.handleTypstLocation = (
    elem: Element,
    page: number,
    x: number,
    y: number,
    options?: any,
  ) => {
    const docRoot = findAncestor(elem, 'typst-app');
    if (!docRoot) {
      console.warn('no typst-app found', elem);
      return;
    }

    console.log(docRoot);

    for (const h of docRoot.children) {
      if (h.classList.contains('typst-dom-page')) {
        const idx = Number.parseInt(h.getAttribute('data-index')!);
        if (idx + 1 === page) {
          const svg = h.querySelector('.typst-svg-page');
          if (svg) {
            // todo: load it.
            baseHandleTypstLocation(svg, 1, x, y, options);
          }
          return;
        }
      }
    }
  };

  window.assignSemaHash = (u: number, x: number, y: number) => {
    // console.log(`find labels ${u}:${x}:${y} in`, globalSemaLabels);
    for (const [label, dom, pos] of globalSemaLabels) {
      const [u1, x1, y1] = pos;
      if (u === u1 && Math.abs(x - x1) < 0.01 && Math.abs(y - y1) < 0.01) {
        location.hash = `label-${label}`;
        // const domX1 = x1 * dom.viewBox.baseVal.width;
        // const domY1 = y1 * dom.viewBox.baseVal.height;

        window.typstCheckAndRerender?.(false, new Error('assignSemaHash')).then(() => {
          const width = dom.viewBox.baseVal.width;
          const height = dom.viewBox.baseVal.height;
          const bbox = dom.getBoundingClientRect();
          const domX1 = bbox.left + (x1 / width) * bbox.width;
          const domY1 = bbox.top + (y1 / height) * bbox.height;

          const lnk = findLinkInSvg(dom, [domX1, domY1]);
          if (!lnk) {
            return;
          }
          // const semaLinkLocation = document.getElementById(`typst-label-${label}`);
          const relatedElems: Element[] = window.typstGetRelatedElements(lnk);
          for (const h of relatedElems) {
            h.classList.add('focus');
          }
          updateHovers(relatedElems);
          return;
        });
        return;
      }
    }
    updateHovers([]);
    // todo: multiple documents
    location.hash = `loc-${u}x${x.toFixed(2)}x${y.toFixed(2)}`;
  };

  async function reloadArtifact(theme: string) {
    // free anyway
    if (session) {
      disposeSession();
    }

    appElem.innerHTML = '';
    // todo: don't modify this attribute here, instead hide detail in typst.ts
    appElem.removeAttribute('data-applied-width');

    const t0 = performance.now();
    const artifactData = await fetch(`${relPath}.${theme}.multi.sir.in`)
      .then(response => response.arrayBuffer())
      .then(buffer => new Uint8Array(buffer));

    const t1 = performance.now();
    return new Promise(resolve => {
      return plugin.runWithSession(sessionRef => {
        return new Promise(async doDisposeSession => {
          disposeSession = doDisposeSession as any;
          session = sessionRef;
          sessionRef.manipulateData({
            action: 'reset',
            data: artifactData,
          });
          const t2 = performance.now();

          await plugin.renderDom({
            renderSession: sessionRef,
            container: appElem,
            pixelPerPt: 3,
            viewport: getViewport(),
          });

          console.log(
            `theme = ${theme}, load artifact took ${t2 - t1} milliseconds, parse artifact took ${
              t2 - t1
            } milliseconds`,
          );

          resolve(undefined);
        });
      });
    });
  }

  reloadArtifact(currTheme).then(() => {
    // const runRender = async () => {
    //   // const t1 = performance.now();
    //   // console.log('hold', session, currTheme);

    //   // todo: bad performance
    //   appElem.style.margin = `0px`;

    //   // todo: merge
    //   await plugin.renderSvg(session!, appElem);

    //   // const t2 = performance.now();
    //   // console.log(
    //   //   `render took ${t2 - t1} milliseconds.`,
    //   //   appElem.getAttribute('data-applied-width'),
    //   // );

    //   const w = appElem.getAttribute('data-applied-width');
    //   if (w) {
    //     const parentWidth = appElem.parentElement!.clientWidth;
    //     const svgWidth = Number.parseInt(w.slice(0, w.length - 2));
    //     // console.log(
    //     //   parentWidth,
    //     //   svgWidth,
    //     //   window.devicePixelRatio,
    //     //   getComputedStyle(appElem).fontSize,
    //     // );
    //     const wMargin = (parentWidth - svgWidth) / 2;
    //     if (wMargin < 0) {
    //       appElem.style.margin = `0px`;
    //     } else {
    //       appElem.style.margin = `0 ${wMargin}px`;
    //     }
    //   }
    // };

    let base: Promise<any> = Promise.resolve();

    let renderResponsive: boolean | undefined = undefined;
    const checkAndRerender = (r: boolean, stack?: any) => {
      if (r !== true && r !== false) {
        throw new Error('invalid responsive');
      }
      if (r === false) {
        renderResponsive = false;
      } else if (renderResponsive !== false) {
        renderResponsive = true;
      }

      if (stack) {
        console.log('submit', stack);
      }
      return (base = base.then(() => queueRerender(stack)));

      async function queueRerender(stack: any) {
        if (renderResponsive === undefined) {
          if (stack) {
            console.log('skip', stack);
          }
          return;
        }
        let responsive = renderResponsive === false ? false : true;
        renderResponsive = undefined;
        const t = performance.now();

        // console.log('ccc', basePos, appPos, rect);

        await plugin.triggerDomRerender({
          renderSession: session!,
          responsive,
          viewport: getViewport(),
        });

        if (stack) {
          console.log('pull render', performance.now() - t, responsive, stack);
        }
      }
    };

    window.typstChangeTheme = () => {
      const nextTheme = getTheme();
      if (nextTheme === currTheme) {
        return base;
      }
      currTheme = nextTheme;

      return (base = base.then(() => reloadArtifact(currTheme)));
    };

    let responsiveTimeout: any = undefined;
    let responsiveTimeout2: any = undefined;
    const responsiveAction = (responsive?: boolean, stack?: any) => {
      stack ||= window.captureStack();
      clearTimeout(responsiveTimeout);
      if (responsive === undefined || responsive === true) {
        responsiveTimeout = setTimeout(() => {
          checkAndRerender(true, stack);
        }, 10);
      }
      if (responsive === undefined || responsive === false) {
        clearTimeout(responsiveTimeout2);
        responsiveTimeout2 = setTimeout(() => {
          clearTimeout(responsiveTimeout);
          checkAndRerender(false, stack);
        }, 200);
      }
    };

    window.addEventListener('resize', () => responsiveAction());
    window.addEventListener('scroll', () => responsiveAction(false));

    window.typstRerender = responsiveAction;
    window.typstCheckAndRerender = checkAndRerender;

    // trigger again to regard user changed theme during first reloading
    window.typstChangeTheme();
  });
};
