using Browser.Html;

namespace Browser.Css;

public sealed class CssParser(string css)
{
    private static readonly Dictionary<string, string> InheritedProperties = new()
    {
        { "font-size", "16px" },
        { "font-style", "normal" },
        { "font-weight", "normal" },
        { "color", "black" }
    };

    private int _index;

    private void SkipWhiteSpace()
    {
        while (_index < css.Length && char.IsWhiteSpace(css[_index])) _index += 1;
    }

    private void SkipOneChar(char ch)
    {
        if (!(_index < css.Length && css[_index] == ch))
            throw new Exception("Parsing error");
        _index += 1;
    }

    private string ParseWord()
    {
        var start = _index;
        while (_index < css.Length)
            if (char.IsAsciiLetterOrDigit(css[_index]) ||
                "#-.%".Contains(css[_index]))
                _index += 1;
            else
                break;
        if (!(_index > start)) throw new Exception("Parsing error");
        return css[start.._index];
    }

    private (string, string) ParsePair()
    {
        var prop = ParseWord().ToLower();
        SkipWhiteSpace();
        SkipOneChar(':');
        SkipWhiteSpace();
        var val = ParseWord();
        return (prop, val);
    }

    private char? SkipUntil(char[] chars)
    {
        while (_index < css.Length)
        {
            if (chars.Contains(css[_index])) return css[_index];

            _index += 1;
        }

        return null;
    }

    private Dictionary<string, string> ParseBody()
    {
        var pairs = new Dictionary<string, string>();
        while (_index < css.Length && css[_index] != '}')
            try
            {
                var (prop, val) = ParsePair();
                pairs[prop] = val;
                SkipWhiteSpace();
                SkipOneChar(';');
                SkipWhiteSpace();
            }
            catch (Exception e)
            {
                var why = SkipUntil([';', '}']);
                if (why == ';')
                {
                    SkipOneChar(';');
                    SkipWhiteSpace();
                }
                else
                {
                    break;
                }
            }

        return pairs;
    }

    private Selector ParseSelector()
    {
        Selector selector = new TagSelector(ParseWord().ToLower());
        SkipWhiteSpace();
        while (_index < css.Length && css[_index] != '}')
        {
            var tagName = ParseWord().ToLower();
            var descendant = new TagSelector(tagName);
            selector = new DescendantSelector(selector, descendant);
            SkipWhiteSpace();
        }

        return selector;
    }

    public List<(Selector, Dictionary<string, string>)> Parse()
    {
        var rules = new List<(Selector, Dictionary<string, string>)>();
        while (_index < css.Length)
            try
            {
                SkipWhiteSpace();
                var selector = ParseSelector();
                SkipOneChar('{');
                SkipWhiteSpace();
                var body = ParseBody();
                SkipOneChar('}');
                rules.Add((selector, body));
            }
            catch (Exception e)
            {
                var why = SkipUntil(['}']);
                if (why == '}')
                {
                    SkipOneChar('}');
                    SkipWhiteSpace();
                }
                else
                {
                    break;
                }
            }

        return rules;
    }

    public static int CascadePriority(
        (Selector, Dictionary<string, string>) rule)
    {
        var (selector, _) = rule;
        return selector.Priority;
    }

    public static void ApplyCss(HtmlNode node,
        List<(Selector, Dictionary<string, string>)> rules)
    {
        foreach (var (property, defaultValue) in InheritedProperties)
            if (node.Parent != null)
                node.Styles[property] = node.Parent.Styles[property];
            else
                node.Styles[property] = defaultValue;
        foreach (var (selector, body) in rules)
        {
            if (!selector.Matches(node)) continue;
            foreach (var (property, value) in body)
                node.Styles[property] = value;
        }

        if (node is HtmlElement htmlElement &&
            htmlElement.Attributes.TryGetValue("style", out var css))
        {
            var pairs =
                new CssParser(css).ParseBody();
            foreach (var (property, value) in pairs)
                node.Styles[property] = value;
        }

        if (node.Styles["font-size"].EndsWith('%'))
        {
            var parentFontSize = "";
            parentFontSize = node.Parent != null
                ? node.Parent.Styles["font-size"]
                : InheritedProperties["font-size"];
            var nodePercentage =
                (float)Convert.ToDouble(node.Styles["font-size"][..^1]) / 100;
            var parentPercentage =
                (float)Convert.ToDouble(parentFontSize[..^2]);
            node.Styles["font-size"] =
                nodePercentage * parentPercentage + "px";
        }

        foreach (var child in node.Children) ApplyCss(child, rules);
    }
}