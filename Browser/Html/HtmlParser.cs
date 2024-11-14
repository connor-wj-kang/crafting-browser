namespace Browser.Html;

public class HtmlParser(string html)
{
    private static readonly string[] HeadTags =
    [
        "base",
        "basefont",
        "bgsound",
        "noscript",
        "link",
        "meta",
        "title",
        "style",
        "script"
    ];

    private static readonly string[] SelfClosingTags =
    [
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
        "wbr"
    ];

    private readonly Stack<HtmlElement> _unfinishedElements = [];

    public HtmlElement Parse()
    {
        var source = "";
        var inTag = false;
        foreach (var c in html.ToCharArray())
            switch (c)
            {
                case '<':
                {
                    inTag = true;
                    if (source != "") ParseText(source);
                    Console.WriteLine(source);
                    source = "";
                    break;
                }
                case '>':
                    inTag = false;
                    ParseElement(source);
                    Console.WriteLine(source);
                    source = "";
                    break;
                default:
                    source += c;
                    break;
            }

        if (!inTag && source != "") ParseText(source);
        return Finish();
    }

    private HtmlElement Finish()
    {
        if (_unfinishedElements.Count == 0) AddImplicitElements();
        while (_unfinishedElements.Count > 1)
        {
            var element = _unfinishedElements.Pop();
            var parent = _unfinishedElements.Peek();
            parent.Children.Add(element);
        }

        return _unfinishedElements.Pop();
    }

    private void AddImplicitElements(string tagName = "")
    {
        string[] tags = ["head", "body", "/html"];
        while (true)
        {
            var openTags =
                _unfinishedElements.Select(element => element.TagName)
                    .ToArray();
            if (openTags is [] && tagName != "html")
                ParseElement("html");
            else if (openTags is ["html"] && !tags.Contains(tagName))
                ParseElement(HeadTags.Contains(tagName) ? "head" : "body");
            else if (openTags is ["head", "html"] && tagName != "/head" &&
                     !HeadTags.Contains(tagName))
                ParseElement("/head");
            else
                break;
        }
    }


    private void ParseElement(string element)
    {
        var (tagName, attributes) = ParseTagNameAndAttributes(element);
        if (tagName.StartsWith('!')) return;
        AddImplicitElements(tagName);
        if (tagName.StartsWith('/'))
        {
            if (_unfinishedElements.Count == 1)
                return;
            var htmlElement = _unfinishedElements.Pop();
            var parent = _unfinishedElements.Peek();
            parent.Children.Add(htmlElement);
        }
        else if (SelfClosingTags.Contains(tagName))
        {
            var parent = _unfinishedElements.Peek();
            var htmlElement = new HtmlElement(attributes, tagName, parent);
            parent.Children.Add(htmlElement);
        }
        else
        {
            _unfinishedElements.TryPeek(out var parent);
            var htmlElement = new HtmlElement(attributes, tagName, parent);
            _unfinishedElements.Push(htmlElement);
        }
    }

    private void ParseText(string text)
    {
        if (string.IsNullOrWhiteSpace(text)) return;
        AddImplicitElements();
        var parent = _unfinishedElements.Peek();
        var htmlText = new HtmlText(text, parent);
        parent.Children.Add(htmlText);
    }

    private static (string, Dictionary<string, string>)
        ParseTagNameAndAttributes(
            string tagContent)
    {
        var parts = tagContent.Split(' ');
        var tagName = parts[0].ToLower();
        var attributes = new Dictionary<string, string>();
        foreach (var attributePair in parts[1..])
        {
            if (attributePair == "/" ||
                string.IsNullOrWhiteSpace(attributePair))
                continue;
            if (attributePair.Contains('='))
            {
                var keyValue = attributePair.Split('=', 2);
                var key = keyValue[0].ToLower();
                var value = keyValue[1];
                if (value.Length > 2 && (value[0] == '\'' || value[0] != '"'))
                    value = value[1..^1];
                attributes[key] = value;
            }
            else
            {
                attributes[attributePair.ToLower()] = "";
            }
        }

        return (tagName, attributes);
    }
}