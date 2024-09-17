const HEAD_TAGS = new Set([
  "base",
  "basefont",
  "bgsound",
  "noscript",
  "link",
  "meta",
  "title",
  "style",
  "script",
]);

const SELF_CLOSING_TAGS = new Set([
  "area",
  "base",
  "br",
  "col",
  "embed",
  "hr",
  "img",
  "input",
  "link",
  "meta",
  "param",
  "source",
  "track",
  "wbr",
]);

export type HtmlNode = HtmlElement | HtmlText;

export class HtmlText {
  text: string;
  parent?: HtmlElement;

  constructor(text: string, parent?: HtmlElement) {
    this.text = text;
    this.parent = parent;
  }
}

export class HtmlElement {
  public tag: string;
  public children: HtmlNode[];
  public attributes: Map<string, string | undefined>;
  public parent?: HtmlElement;

  constructor(
    tag: string,
    children: HtmlNode[],
    attributes: Map<string, string | undefined>,
    parent?: HtmlElement,
  ) {
    this.tag = tag;
    this.children = children;
    this.attributes = attributes;
    this.parent = parent;
  }
}

class HtmlParser {
  body: string;
  openTagsStack: HtmlElement[];

  constructor(body: string) {
    this.body = body;
    this.openTagsStack = [];
  }

  parse(): HtmlElement {
    let text = "";
    let inTag = false;

    for (const char of this.body) {
      switch (char) {
        case "<":
          inTag = true;
          if (text.length !== 0) {
            this.addHtmlText(text);
          }
          text = "";
          break;
        case ">":
          inTag = false;
          this.addHtmlElement(text);
          text = "";
          break;
        default:
          text += char;
      }
    }

    if (!inTag && text.length !== 0) {
      this.addHtmlText(text);
    }

    return this.finish();
  }

  addHtmlText(text: string) {
    if (text.trim() === "") {
      return;
    }

    this.addImplicitTags();

    const parent = this.openTagsStack.at(this.openTagsStack.length - 1);
    const node = new HtmlText(text, parent);
    parent?.children.push(node);
  }

  addHtmlElement(openTagContent: string) {
    const [tag, attributes] = this.getAttributes(openTagContent);

    if (tag.startsWith("!")) {
      return;
    }

    this.addImplicitTags(tag);

    if (tag.startsWith("/") && this.openTagsStack.length > 1) {
      const newHtmlElement = this.openTagsStack.pop()!;
      const parent = this.openTagsStack.slice(-1)[0];
      parent.children.push(newHtmlElement);
      return;
    }

    if (SELF_CLOSING_TAGS.has(tag)) {
      const newHtmlElement = this.openTagsStack.at(
        this.openTagsStack.length - 1,
      );
      const node = new HtmlElement(tag, [], attributes, newHtmlElement);
      newHtmlElement?.children.push(node);
      return;
    }

    const parent = this.openTagsStack.at(this.openTagsStack.length - 1);
    const newHtmlElement = new HtmlElement(tag, [], attributes, parent);
    this.openTagsStack.push(newHtmlElement);
  }

  addImplicitTags(tag: string = "") {
    while (true) {
      const openTags = this.openTagsStack.map((openTag) => openTag.tag);

      if (openTags.length === 0 && tag !== "html") {
        this.addHtmlElement("html");
        continue;
      }

      if (
        openTags.length === 1 &&
        openTags[0] === "html" &&
        !["head", "body", "/html"].includes(tag)
      ) {
        if (HEAD_TAGS.has(tag)) {
          this.addHtmlElement("head");
        } else {
          this.addHtmlElement("body");
        }
        continue;
      }

      if (
        openTags.length === 2 &&
        openTags[0] === "html" &&
        openTags[1] === "head" &&
        !HEAD_TAGS.has(tag) &&
        tag !== "/head"
      ) {
        this.addHtmlElement("/head");
        continue;
      }

      break;
    }
  }

  finish(): HtmlElement {
    if (this.openTagsStack.length !== 0) {
      this.addImplicitTags();
    }

    while (this.openTagsStack.length > 1) {
      const node = this.openTagsStack.pop()!;
      const parent = this.openTagsStack.slice(-1)[0];
      parent.children.push(node);
    }

    return this.openTagsStack.pop()!;
  }

  getAttributes(
    openTagContent: string,
  ): [tag: string, attributes: Map<string, string | undefined>] {
    const attributes = new Map();
    const [tag, ...attributesPair] = openTagContent.split(" ");

    attributesPair.slice(1).forEach((attributesPair) => {
      if (attributesPair.includes("=")) {
        const keyAndValue = attributesPair.split("=", 2);
        const key = keyAndValue[0];
        let value = keyAndValue[1];

        if (
          value.length > 2 &&
          (value.startsWith("'") || value.startsWith('"'))
        ) {
          value = value.slice(1, -1);
        }

        attributes.set(key, value);
      } else {
        attributes.set(attributesPair, undefined);
      }
    });

    return [tag, attributes];
  }
}

export default HtmlParser;
