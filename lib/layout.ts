import { HtmlElement, HtmlNode, HtmlText } from "./html-parser";

const BLOCK_ELEMENTS = new Set([
  "html",
  "body",
  "article",
  "section",
  "nav",
  "aside",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hgroup",
  "header",
  "footer",
  "address",
  "p",
  "hr",
  "pre",
  "blockquote",
  "ol",
  "ul",
  "menu",
  "li",
  "dl",
  "dt",
  "dd",
  "figure",
  "figcaption",
  "main",
  "div",
  "table",
  "form",
  "fieldset",
  "legend",
  "details",
  "summary",
]);

type DisplayInfo = {
  x: number;
  y: number;
  text: string;
  fontSize: number;
  fontWeight: "normal" | "bold";
  fontStyle: "normal" | "italic";
};

type Layout = DocumentLayout | BlockLayout;

class DocumentLayout {}

class BlockLayout {
  node: HtmlNode;
  previous?: BlockLayout;
  children: BlockLayout[];
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  parent?: Layout;
  cursorX?: number;
  cursorY?: number;
  fontWeight?: "normal" | "bold";
  fontStyle?: "normal" | "italic";
  fontSize?: number;
  lineBuffer?: DisplayInfo[];
  displayList?: DisplayInfo[];
  static canvas = document.getElementById("canvas") as HTMLCanvasElement;
  static context = this.canvas.getContext("2d")!;

  constructor(node: HtmlNode, parent?: Layout, previous?: BlockLayout) {
    this.node = node;
    this.previous = previous;
    this.children = [];
    this.parent = parent;
  }

  layoutMode(): "inline" | "block" {
    if (this.node instanceof HtmlText) {
      return "inline";
    }

    if (
      this.node.children.some(
        (child) =>
          child instanceof HtmlElement && BLOCK_ELEMENTS.has(child.tag),
      )
    ) {
      return "block";
    }

    if (this.node.children.length !== 0) {
      return "inline";
    }

    return "block";
  }

  layout() {
    const mode = this.layoutMode();

    if (mode === "block") {
      let previous: BlockLayout | undefined = undefined;
      if (this.node instanceof HtmlElement) {
        for (const child of this.node.children) {
          const next: BlockLayout = new BlockLayout(child, this, previous);
          this.children.push(next);
          previous = next;
        }
      }
    }

    if (mode === "inline") {
      this.cursorX = 0;
      this.cursorY = 0;
      this.fontWeight = "normal";
      this.fontStyle = "normal";
      this.fontSize = 12;
      this.lineBuffer = [];
    }

    this.children.forEach((child) => child.layout());
  }

  openTag(tag: string) {
    switch (tag) {
      case "i":
        this.fontStyle = "italic";
        break;
      case "b":
        this.fontWeight = "bold";
        break;
      case "small":
        this.fontSize = (this.fontSize ?? 12) - 4.0;
        break;
      case "big":
        this.fontSize = (this.fontSize ?? 12) + 4.0;
        break;
    }
  }

  closeTag(tag: string) {
    switch (tag) {
      case "i":
        this.fontStyle = "normal";
        break;
      case "b":
        this.fontWeight = "normal";
        break;
      case "small":
        this.fontSize = (this.fontSize ?? 12) + 4.0;
        break;
      case "big":
        this.fontSize = (this.fontSize ?? 12) - 4.0;
        break;
    }
  }

  recurse(tree: HtmlNode) {
    if (tree instanceof HtmlText) {
      BlockLayout.context.font = `${this.fontStyle} ${this.fontWeight} ${this.fontSize}px serif`;
      const spaceWidth = BlockLayout.context.measureText(" ").width;
      tree.text.split(" ").forEach((word) => {
        const wordWidth = BlockLayout.context.measureText(word).width;
        this.lineBuffer?.push({
          x: this.cursorX!,
          y: 0,
          text: word,
          fontStyle: this.fontStyle!,
          fontSize: this.fontSize!,
          fontWeight: this.fontWeight!,
        });

        if (this.cursorX! + wordWidth > BlockLayout.canvas.width) {
          this.flushLineBuffer();
        } else {
          this.cursorX! += wordWidth + spaceWidth;
        }
      });
    }

    if (tree instanceof HtmlElement) {
      this.openTag(tree.tag);
      tree.children.forEach((child) => this.recurse(child));
      this.closeTag(tree.tag);
    }
  }

  flushLineBuffer() {
    if (this.lineBuffer?.length === 0) {
      return;
    }

    const [maxAscent, maxDescent] = (this.lineBuffer ?? [])
      .map((displayInfo) => {
        BlockLayout.context.font = `${this.fontStyle} ${this.fontWeight} ${this.fontSize}px serif`;
        const fontMetric = BlockLayout.context.measureText(displayInfo.text);
        return [
          fontMetric.fontBoundingBoxAscent,
          fontMetric.fontBoundingBoxDescent,
        ];
      })
      .reduce(
        ([ascent, descent], [x, y]) => [
          Math.max(ascent, x),
          Math.max(descent, y),
        ],
        [Number.NEGATIVE_INFINITY, Number.NEGATIVE_INFINITY],
      );

    const baseLine = this.cursorY! + 1.25 * maxAscent;
    this.lineBuffer?.forEach((displayInfo) =>
      this.displayList?.push({
        ...displayInfo,
        y: baseLine,
      }),
    );

    this.cursorX = 0;
    this.cursorY = baseLine + 1.25 * maxDescent;
    this.lineBuffer = [];
  }
}
