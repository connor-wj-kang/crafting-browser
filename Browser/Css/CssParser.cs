using Browser.Html;

namespace Browser.Css;

public class CssParser(string css)
{
    public static Dictionary<string, string> InheritedProperties = new()
    {
        { "font-size", "16px" },
        { "font-style", "normal" },
        { "font-weight", "normal" },
        { "color", "black" }
    };

    public readonly string Css = css;
    public int Index;

    private void SkipWhiteSpace()
    {
        while (Index < Css.Length && char.IsWhiteSpace(Css[Index])) Index += 1;
    }

    private void SkipOneChar(char ch)
    {
        if (!(Index < Css.Length && Css[Index] == ch))
            throw new Exception("Parsing error");
        Index += 1;
    }

    private string ParseWord()
    {
        var start = Index;
        while (Index < Css.Length)
            if (char.IsAsciiLetterOrDigit(Css[Index]) ||
                "#-.%".Contains(Css[Index]))
                Index += 1;
            else
                break;
        if (!(Index > start)) throw new Exception("Parsing error");
        return Css[start..Index];
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
        while (Index < Css.Length)
        {
            if (chars.Contains(Css[Index])) return Css[Index];

            Index += 1;
        }

        return null;
    }

    private Dictionary<string, string> ParseBody()
    {
        var pairs = new Dictionary<string, string>();
        while (Index < Css.Length && Css[Index] != '}')
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
        while (Index < Css.Length && Css[Index] != '}')
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
        while (Index < Css.Length)
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