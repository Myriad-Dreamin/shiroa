import { TypstRenderer } from '@myriaddreamin/typst.ts/dist/esm/renderer';

declare global {
  interface Window {
    typstPathToRoot: string | undefined;
    typstGetRelatedElements: any;
    handleTypstLocation: any;
    getTypstTheme(): string;
    captureStack(): any;
    typstRerender?: (responsive?: boolean) => void;
    typstCheckAndRerender?: (responsive: boolean, stack?: any) => Promise<void>;
    typstChangeTheme?: () => Promise<void>;
    debounce<T extends { (...args: any[]): void }>(fn: T, delay = 200): T;
    assignSemaHash: (u: number, x: number, y: number) => void;
    typstProcessSvg: any;
    typstBookRenderPage(
      plugin: TypstSvgRenderer,
      relPath: string,
      appContainer: HTMLDivElement | undefined,
    );
    typstBindSvgDom(elem: HTMLDivElement, dom: SVGSVGElement);
    TypstRenderModule: any;
  }
}
