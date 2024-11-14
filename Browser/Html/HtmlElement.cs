namespace Browser.Html;

public class HtmlElement : HtmlNode
{
    public readonly Dictionary<string, string> Attributes;
    public readonly string TagName;

    public HtmlElement(Dictionary<string, string> attributes, string tagName,
        HtmlNode? parent = null)
    {
        Attributes = attributes;
        TagName = tagName;
        Parent = parent;
    }
}