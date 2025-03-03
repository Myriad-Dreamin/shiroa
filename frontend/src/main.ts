//#/dev/frontend/dist/book.mjs
import { kObject } from '@myriaddreamin/typst.ts/dist/esm/internal.types.mjs';
import {
  RenderSession,
  TypstRenderer,
  createTypstRenderer,
} from '@myriaddreamin/typst.ts/dist/esm/renderer.mjs';
import type { TypstDomDocument } from '@myriaddreamin/typst.ts/dist/esm/dom.mjs';

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
let initialRender = true;
let jumppedCrossLink = false;
function postProcessCrossLinks(appElem: HTMLDivElement, reEnters: number) {
  const links = appElem.querySelectorAll('.typst-content-link');
  if (links.length === 0) {
    console.log('no links found, probe after a while');
    setTimeout(() => postProcessCrossLinks(appElem, reEnters * 1.5), reEnters);
    return;
  }
  links.forEach(a => {
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

  // todo: out of page
  if (window.location.hash && !jumppedCrossLink) {
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
          jumppedCrossLink = true;
          break;
        }
      }
    }
  }
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

// todo: split html frontend js and paged frontend js
window.typstBookRenderHtmlPage = function (
  relPath: string,
  appContainer: HTMLDivElement | undefined,
) {
  // todo: preload artifact
  const getTheme = () => window.getTypstTheme();
  let currTheme = getTheme();

  async function reloadArtifact(theme: string) {
    const preloadContent = appContainer?.querySelector('.typst-preload-content')!;
    if (!preloadContent) {
      console.error('no preload content found');
      return;
    }

    preloadContent.innerHTML = '';
    // todo: don't modify this attribute here, instead hide detail in typst.ts
    preloadContent.removeAttribute('data-applied-width');

    const artifactData = await fetch(`${relPath}.${theme}.html`).then(response => response.text());

    const themePreloadContent = document.createElement('div');
    themePreloadContent.className = 'typst-preload-content';
    themePreloadContent.innerHTML = artifactData;

    preloadContent.replaceWith(themePreloadContent);
    themePreloadContent.style.display = 'block';
  }

  reloadArtifact(currTheme).then(() => {
    let base: Promise<any> = Promise.resolve();

    window.typstChangeTheme = () => {
      const nextTheme = getTheme();
      if (nextTheme === currTheme) {
        return base;
      }
      currTheme = nextTheme;

      return reloadArtifact(currTheme);
    };

    // trigger again to regard user changed theme during first reloading
    window.typstChangeTheme();
  });
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
  let dom: TypstDomDocument = undefined!;
  let disposeSession: () => void = () => {};

  const appElem = document.createElement('div');
  if (appElem && appContainer) {
    appElem.className = 'typst-app';
    appContainer.appendChild(appElem);
  }

  const dec = new TextDecoder();
  window.typstBindSvgDom = async (_elem: HTMLDivElement, _dom: SVGSVGElement) => {};

  let runningSemantics: Record<string, string> = {};
  const typstBindCustomSemantics = async (
    root: HTMLElement,
    svg: SVGSVGElement,
    semantics: HTMLDivElement,
  ) => {
    const index = root?.getAttribute('data-index');
    const key = `${index}`;
    const width = root?.getAttribute('data-width');
    const keyResolving = `${width}`;
    if (runningSemantics[key] === keyResolving) {
      return;
    }
    runningSemantics[key] = keyResolving;
    console.log('bind custom semantics', key, keyResolving, svg?.viewBox);
    const customs = await plugin.getCustomV1({
      renderSession: session!,
    });
    const semaLabel = customs.find((k: [string, string]) => k[0] === 'sema-label');
    if (semaLabel) {
      const labelBin = semaLabel[1];
      const labels = JSON.parse(dec.decode(labelBin));
      globalSemaLabels = labels.map(([label, pos]: [string, string]) => {
        const [_, u, x, y] = pos.split(/[pxy]/).map(Number.parseFloat);
        return [encodeURIComponent(label), svg, [u, x, y]];
      });
    }

    postProcessCrossLinks(semantics, 100);
  };
  // todo: remove this hack
  let semanticHandlers: (() => void)[] = [];
  (window as any).typstBindCustomSemantics = (
    root: HTMLElement,
    svg: SVGSVGElement,
    semantics: HTMLDivElement,
  ) =>
    setTimeout(() => {
      const semanticHandler = () => {
        typstBindCustomSemantics(root, svg, semantics);
      };
      semanticHandler();
      semanticHandlers.push(semanticHandler);
    }, 0);

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
    options = options || {};
    options.isDom = true;

    for (const h of docRoot.children) {
      if (h.classList.contains('typst-dom-page')) {
        const idx = Number.parseInt(h.getAttribute('data-index')!);
        if (idx + 1 === page) {
          const svg = h.querySelector('.typst-svg-page');
          if (svg) {
            baseHandleTypstLocation(svg, page, x, y, options);
          }
          return;
        }
      }
    }
  };

  window.assignSemaHash = (u, x, y) => {
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
          const relatedElems = window.typstGetRelatedElements(lnk);
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
    if (dom) {
      dom.dispose();
      dom = undefined!;
    }
    if (session) {
      disposeSession();
      session = undefined!;
    }

    appElem.innerHTML = '';
    // todo: don't modify this attribute here, instead hide detail in typst.ts
    appElem.removeAttribute('data-applied-width');

    const artifactData = await fetch(`${relPath}.${theme}.multi.sir.in`)
      .then(response => response.arrayBuffer())
      .then(buffer => new Uint8Array(buffer));

    const t1 = performance.now();
    return new Promise(resolve => {
      return plugin.runWithSession(sessionRef => {
        return new Promise(async doDisposeSession => {
          disposeSession = doDisposeSession as any;
          session = sessionRef;
          const t2 = performance.now();

          jumppedCrossLink = false;
          semanticHandlers.splice(0, semanticHandlers.length);
          runningSemantics = {};
          dom = await plugin.renderDom({
            renderSession: sessionRef,
            container: appElem,
            pixelPerPt: 4.5,
          });
          const mod = dom.impl.modes.find(([k, _]) => k == 'dom')!;
          const postRender = mod[1].postRender;
          mod[1].postRender = function () {
            console.log('hack run semantic handlers');
            postRender.apply(this);
            for (const h of semanticHandlers) {
              h();
            }
            return;
          };

          console.log(
            `theme = ${theme}, load artifact took ${t2 - t1} milliseconds, parse artifact took ${
              t2 - t1
            } milliseconds`,
          );

          dom.addChangement(['new', artifactData as unknown as string]);

          resolve(dom);
        });
      });
    });
  }

  reloadArtifact(currTheme).then((dom: TypstDomDocument) => {
    let base: Promise<any> = Promise.resolve();

    window.typstChangeTheme = () => {
      const nextTheme = getTheme();
      if (nextTheme === currTheme) {
        return base;
      }
      currTheme = nextTheme;

      return reloadArtifact(currTheme);
    };

    const viewportHandler = () => dom.addViewportChange();
    window.addEventListener('resize', viewportHandler);
    window.addEventListener('scroll', viewportHandler);
    dom.impl.disposeList.push(() => {
      window.removeEventListener('resize', viewportHandler);
      window.removeEventListener('scroll', viewportHandler);
    });
    window.typstRerender = viewportHandler;

    // trigger again to regard user changed theme during first reloading
    window.typstChangeTheme();
  });
};
